use std::env::var;
use std::time::SystemTime;
use crate::_row_to_string;
use crate::database::connect;

pub fn get_cells(row: &postgres::Row) -> Vec<String> {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            },
            "int4" => {
                let value: Option<i32> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            },

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
    cells
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
        println!("{}", _row_to_string(&row));
    }
}

fn run_database(database_name: String) {
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
        println!("{}", _row_to_string(&row));
    }

    client.close().unwrap();
}

pub fn different_row_count() {

}