use std::env::var;
use postgres::{Client, Error, NoTls};

/**
 * Connect to the database
 *
 * Author : Ta Quang Khoi
 */
pub fn connect(database_name: String) -> rusqlite::Result<Client, Error> {
    let username = var("POSTGRES_USER").unwrap_or(String::from(""));
    let password = var("POSTGRES_PASSWORD").unwrap_or(String::from(""));
    let host = var("POSTGRES_HOST").unwrap_or(String::from(""));

    let database_url = format!("postgresql://{}:{}@{}/{}", username, password, host, database_name);

    let client = Client::connect(
        database_url.as_str(),
        NoTls,
    )?;

    Ok(client)
}