pub mod database;
mod preparation;
mod table;
mod core;
mod working_database;

use database::connect;
use std::env::var;
use postgres::error::SqlState;
use rusqlite::{params, Connection};
use crate::core::get_tables;
use crate::preparation::prepare_knowledge;
use crate::working_database::get_cells;

fn main() {
    prepare_knowledge();
    compare_database();
    // export based on complexity
}

fn compare_database() {
    let source_database_name = var("POSTGRES_DB_1").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_2").unwrap_or(String::from(""));
    let mut source_client = connect(source_database_name.clone()).unwrap();
    let mut target_client = connect(target_database_name.clone()).unwrap();

    let sqlite_conn = Connection::open("twodb.db").unwrap();
    let tables_to_compare = get_tables(&sqlite_conn);

    for table in tables_to_compare {
        let table_name = table.name.clone();
        let query = "SELECT * FROM ".to_string() + table_name.as_str();
        let source_rows = match source_client.query(&query, &[]) {
            Ok(rows) => rows,
            Err(err) => {
                if let Some(db_err) = err.as_db_error() {
                    if db_err.code() == &SqlState::from_code("42P01") {
                        println!("Table: {} does not exist in the source database", table_name);
                        continue;
                    }
                }
                panic!("Error querying source database: {:?}", err);
            }
        };
        let target_rows = match target_client.query(&query, &[]) {
            Ok(rows) => rows,
            Err(err) => {
                if let Some(db_err) = err.as_db_error() {
                    if db_err.code() == &SqlState::from_code("42P01") {
                        println!("Table: {} does not exist in the target database", table_name);
                        continue;
                    }
                }
                panic!("Error querying target database: {:?}", err);
            }
        };

        let source_rows_count = source_rows.len();
        let target_rows_count = target_rows.len();
        if source_rows_count != target_rows_count {
            println!("Table: {} has different rows count: {} vs {}", table_name, source_rows_count, target_rows_count);
            continue;
        }

        for (source_row, target_row) in source_rows.iter().zip(target_rows.iter()) {
            let source_cells = get_cells(source_row);

            let target_cells = get_cells(target_row);

            if source_cells != target_cells {
                println!("Table: {} has different cells", table_name);
                println!("Source: {:?}", source_cells);
                println!("Target: {:?}", target_cells);
            }
        }
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

fn _row_to_string(row: &postgres::Row) -> String {
    format!("{:?}", get_cells(row))
}