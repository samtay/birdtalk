use dotenvy_macro::dotenv;

pub const SUPABASE_API_URL: &str = dotenv!("SUPABASE_API_URL");
pub const SUPABASE_ANON_KEY: &str = dotenv!("SUPABASE_ANON_KEY");
pub const SUPABASE_STORAGE_URL: &str = dotenv!("SUPABASE_STORAGE_URL");
