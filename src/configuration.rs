pub use anyhow::Result;

#[derive(serde::Deserialize)]
pub struct Settings<'a> {
    pub application_port: u16,
    pub db_name: &'a str,
}
