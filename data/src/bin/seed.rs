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
    common_name: String,
    default_sound_file: Option<String>,
    default_image_file: Option<String>,
}

#[derive(serde::Deserialize)]
struct Bird {
    scientific_name: String,
    common_name: String,
    default_sound_file: String,
    img_file: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    upload_media().await?;
    Ok(())
}

async fn upload_media() -> Result<()> {
    // pull seed data from filesystem
    let seed_dir = env::var("SEED_DIR")?;
    let file = std::fs::File::open(format!("{seed_dir}/birds.json"))?;
    let bird_seed: HashMap<String, Bird> = serde_json::from_reader::<_, Vec<Bird>>(file)?
        .into_iter()
        .map(|bird| (bird.scientific_name.clone(), bird))
        .collect();

    // pull data from database
    let db_url = env::var("DATABASE_URL")?;
    let mut conn = PgConnection::connect(&db_url).await?;
    let db_birds = sqlx::query_as!(
        BirdRow,
        r#"select
            birds.id as "id!",
            birds.scientific_name as "scientific_name!",
            birds.common_name as "common_name!",
            sounds.bucket || '/' || sounds.path as default_sound_file,
            images.bucket || '/' || images.path as default_image_file
         from birds
         left join bird_images images on birds.id = images.bird
         left join bird_sounds sounds on birds.id = sounds.bird"#,
    )
    .fetch_all(&mut conn)
    .await?;

    // upload media
    let sb_env = env::var("ENV")?;
    let flag = match sb_env.as_str() {
        "local" => "--local",
        "staging|production" => "--linked",
        _ => unimplemented!(),
    };
    let sb_args = vec!["--experimental", "storage", flag, "cp", "--recursive"];

    for bird in db_birds {
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
            .execute(&mut conn)
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
            .execute(&mut conn)
            .await?;
        }
    }

    Ok(())
}
