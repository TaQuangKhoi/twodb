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
        println!("{}", row_to_string(row));
    }
}

fn row_to_string(row: postgres::Row) -> String {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        println!("{}: {:?}", name, type_);
        match type_ {
            INT8 => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            }
            Varchar => {
                let value: Option<&str> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or("None"))
            }
        }
    }).collect();

    format!("{:?}", cells)
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
