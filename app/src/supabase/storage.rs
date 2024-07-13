// use std::sync::OnceLock;
// static SB_CLIENT: OnceLock<String> = OnceLock::new();

use crate::conf::SUPABASE_API_URL;

pub fn storage_object_url<S: AsRef<str>>(path: S) -> String {
    format!(
        "{SUPABASE_API_URL}/storage/v1/object/public/{}",
        path.as_ref()
    )
}
