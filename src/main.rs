use postgres::{Client, Error, NoTls};

fn main() {
    let mut client = connect().unwrap();
    let query = "___YOUR_QUERY_HERE___";
    let rows = client.query(
        query,
        &[],
    ).unwrap();

    for row in rows {
        let id: Option<i64> = row.try_get(0).unwrap_or(None); // bigserial
        let name: Option<&str> = row.try_get(1).unwrap_or(None);
        let email: Option<&str> = row.try_get(2).unwrap_or(None);

        let mut raw_id: i64 = None.unwrap_or(0);
        let mut raw_name: &str = None.unwrap_or("None");
        let mut raw_email: &str = None.unwrap_or("None");
        match id {
            Some(id) => raw_id = id,
            None => raw_id = 0,
        }
        match name {
            Some(name) => raw_name = name,
            None => raw_name = "None",
        }
        match email {
            Some(email) => raw_email = email,
            None => raw_email = "None",
        }
        println!("id: {}, name: {}, email: {}", raw_id, raw_name, raw_email)
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
        NoTls,
    )?;

    Ok(client)
}
