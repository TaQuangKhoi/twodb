use postgres::{Client, Error, NoTls};

fn main() {
    let mut client = connect().unwrap();
    let query = "___YOUR_QUERY_HERE___";
    let rows = client.query(query, &[]).unwrap();

    for row in rows {
        let id: Option<i64> = row.try_get(0).unwrap_or(None);
        let name: Option<&str> = row.try_get(1).unwrap_or(None);
        let email: Option<&str> = row.try_get(2).unwrap_or(None);

        println!("id: {:?}, name: {:?}, email: {:?}", id, name, email);
    }
}

fn connect() -> Result<Client, Error> {
    let database_name = String::from("");
    let username = String::from("");
    let password = String::from("");
    let host = String::from("");
    let database_url = format!("postgresql://{}:{}@{}/{}", username, password, host, database_name);

    let client = Client::connect(
        database_url.as_str(),
        NoTls
    )?;

    Ok(client)
}