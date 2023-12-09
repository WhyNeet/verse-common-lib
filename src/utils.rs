use std::env;

pub fn get_server_port() -> anyhow::Result<u16> {
    Ok(env::var("PORT")
        .map(|val| val.parse())
        .unwrap_or(Ok(8080))?)
}
