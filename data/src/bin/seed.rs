use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process::Command,
};

use anyhow::Result;
use sqlx::{Connection, PgConnection};

#[derive(sqlx::FromRow)]
struct BirdRow {
    id: i32,
    scientific_name: String,
    default_sound_file: Option<String>,
    default_image_file: Option<String>,
}

#[derive(serde::Deserialize)]
struct BirdSeed {
    scientific_name: String,
    common_name: String,
    default_sound_file: String,
    img_file: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // pull seed data from filesystem
    let seed_dir = env::var("SEED_DIR")?;
    let file = std::fs::File::open(format!("{seed_dir}/birds.json"))?;
    let bird_seed: HashMap<String, BirdSeed> = serde_json::from_reader::<_, Vec<BirdSeed>>(file)?
        .into_iter()
        .map(|bird| (bird.scientific_name.clone(), bird))
        .collect();

    // connect to db
    let db_url = env::var("DATABASE_URL")?;
    let mut conn = PgConnection::connect(&db_url).await?;

    upsert_birds(&bird_seed, &mut conn).await?;
    upload_media(&bird_seed, &mut conn).await?;
    Ok(())
}

async fn upsert_birds(
    bird_seed: &HashMap<String, BirdSeed>,
    conn: &mut PgConnection,
) -> Result<()> {
    println!("Upserting birds...");
    let result = sqlx::query!(
        "insert into birds (scientific_name, common_name)
         select * from unnest($1::text[], $2::text[])
         on conflict (scientific_name) do nothing
        ",
        &bird_seed
            .values()
            .map(|bird| bird.scientific_name.clone())
            .collect::<Vec<String>>(),
        &bird_seed
            .values()
            .map(|bird| bird.common_name.clone())
            .collect::<Vec<String>>(),
    )
    .execute(conn)
    .await?;
    println!("Done! Inserted {} birds.", result.rows_affected());

    Ok(())
}

async fn upload_media(
    bird_seed: &HashMap<String, BirdSeed>,
    conn: &mut PgConnection,
) -> Result<()> {
    let seed_dir = env::var("SEED_DIR")?;

    println!("Upserting storage buckets...");
    // first insert buckets if necessary
    sqlx::query!(
        r#"insert into storage.buckets
          (id, name, public)
        values
          ('bird_images', 'bird_images', true),
          ('bird_sounds', 'bird_sounds', true)
        on conflict (id)
        do update set public = true"#
    )
    .execute(&mut *conn)
    .await?;

    println!("Fetching birds from db...");
    let db_birds = sqlx::query_as!(
        BirdRow,
        r#"select
            birds.id as "id!",
            birds.scientific_name as "scientific_name!",
            sounds.bucket || '/' || sounds.path as default_sound_file,
            images.bucket || '/' || images.path as default_image_file
         from birds
         left join bird_images images on birds.id = images.bird
         left join bird_sounds sounds on birds.id = sounds.bird"#,
    )
    .fetch_all(&mut *conn)
    .await?;

    let sb_env = env::var("ENV")?;
    let flag = match sb_env.as_str() {
        "local" => "--local",
        "staging" | "production" => "--linked",
        _ => unimplemented!(),
    };
    let sb_args = vec!["--experimental", "storage", flag, "cp", "--recursive"];

    println!("Uploading media...");
    let total_birds = db_birds.len();
    for (ix, bird) in db_birds.into_iter().enumerate() {
        let bird_seed = &bird_seed
            .get(&bird.scientific_name)
            .expect("missing bird in seed data");
        if bird.default_sound_file.is_none() {
            // Upload media to supabase storage
            let seed_path = &bird_seed.default_sound_file;
            let local_path = format!("{seed_dir}/sounds/{seed_path}");
            let remote_path = format!("ss:///bird_sounds/{seed_path}");
            let mut storage_cmd = Command::new("supabase");
            storage_cmd.args(&sb_args).arg(local_path).arg(remote_path);
            let output = storage_cmd.output()?;
            if !output.status.success() {
                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
                anyhow::bail!("failed to upload sound file");
            }
            // Add media link to database
            sqlx::query!(
                "insert into bird_sounds (bird, bucket, path, default_)
                values ($1, 'bird_sounds', $2, true)",
                bird.id,
                seed_path
            )
            .execute(&mut *conn)
            .await?;
        }
        if bird.default_image_file.is_none() {
            // Upload media to supabase storage
            let seed_path = &bird_seed.img_file;
            let local_path = format!("{seed_dir}/images/{seed_path}");
            let remote_path = format!("ss:///bird_images/{seed_path}");
            let mut storage_cmd = Command::new("supabase");
            storage_cmd.args(&sb_args).arg(local_path).arg(remote_path);
            let output = storage_cmd.output()?;
            if !output.status.success() {
                io::stdout().write_all(&output.stdout)?;
                io::stderr().write_all(&output.stderr)?;
                anyhow::bail!("failed to upload img file");
            }
            // Add media link to database
            sqlx::query!(
                "insert into bird_images (bird, bucket, path, default_)
                values ($1, 'bird_images', $2, true)",
                bird.id,
                seed_path
            )
            .execute(&mut *conn)
            .await?;
        }
        print!("\u{001b}[1000D Completed {}/{} birds", ix + 1, total_birds);
        io::stdout().flush().ok();
    }

    println!("\n\nDone!");
    Ok(())
}
