use postgres::{Client, Error, NoTls};
use std::env::var;
use std::time::SystemTime;

fn main() {
    let source_database_name = var("POSTGRES_DB_1").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_2").unwrap_or(String::from(""));

    get_clean_tables(source_database_name);
}

fn get_clean_tables(database_name: String) {
    let mut client = connect(database_name).unwrap();
    let query ="SELECT table_name \
        FROM information_schema.tables \
        WHERE table_schema = 'public' \
        AND table_type = 'BASE TABLE' \
        AND table_name NOT IN ( \
            SELECT DISTINCT table_name \
            FROM information_schema.table_constraints \
            WHERE constraint_type = 'FOREIGN KEY' \
            AND table_schema = 'public' \
        );".to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    for row in rows {
        println!("{}", row_to_string(row));
    }
}

fn get_all_tables(database_name: String) {
    let mut client = connect(database_name).unwrap();
    let query =
        "SELECT table_name, table_type \
        FROM information_schema.tables \
        WHERE table_schema = 'public'"
            .to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    let base_tables: Vec<String> = rows.iter().map(|row| {
        let table_name: String = row.get(0);
        let table_type: String = row.get(1);
        if table_type == "BASE TABLE" {
            table_name
        } else {
            Some(String::from("")).unwrap()
        }
    }).filter(|table_name| table_name.len() > 0).collect();

    println!("Total base tables: {}", base_tables.len());

    for row in rows {
        println!("{}", row_to_string(row));
    }
}

fn run_database(database_name: String) {
    println!("Database: {}", database_name);

    let mut client = match connect(database_name) {
        Ok(client) => client,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let table_name = var("TABLE_NAME").unwrap_or(String::from(""));
    let query = "SELECT * FROM ".to_string() + table_name.as_str();
    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    for row in rows {
        println!("{}", row_to_string(row));
    }

    client.close().unwrap();
}

fn row_to_string(row: postgres::Row) -> String {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            }
            "varchar" => {
                let value: Option<&str> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or("None"))
            }
            "bool" => {
                let value: Option<bool> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(false))
            }
            "timestamp" => {
                let value: Option<SystemTime> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(SystemTime::UNIX_EPOCH)
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
            }
            _ => {
                println!("Unknown type: {:?}", type_.name());
                format!("{}: {}", name, "Unknown")
            }
        }
    }).collect();

    format!("{:?}", cells)
}

/**
 * Connect to the database
 *
 * Author : Ta Quang Khoi
 */
fn connect(database_name: String) -> Result<Client, Error> {
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
