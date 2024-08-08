use postgres::{Client, Error, NoTls};

fn main() {
    let mut client = match connect() {
        Ok(client) => client,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
let query = "___YOUR_QUERY_HERE___";
    let rows = client.query(
        query,
        &[],
    ).unwrap();

    for row in rows {
        let id: Option<i64> = row.try_get(0).unwrap_or(None); // bigserial
        let name: Option<&str> = row.try_get(1).unwrap_or(None);
        let email: Option<&str> = row.try_get(2).unwrap_or(None);

        let raw_id: i64;
        match id {
            Some(id) => raw_id = id,
            None => raw_id = 0,
        }

        let raw_name: &str;
        match name {
            Some(name) => raw_name = name,
            None => raw_name = "None",
        }

        let raw_email: &str;
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
