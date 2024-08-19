use std::any::Any;
use std::collections::HashMap;
use std::env::var;
use std::time::SystemTime;
use postgres::error::SqlState;
use postgres::{Column, Row};
use crate::core::get_knowledge::get_tables;
use crate::database::connect;

const SQLITE_DATABASE_PATH: &str = "twodb.db";

/// Compare two databases (PostgreSQL)
fn compare_database() {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    let mut source_client = connect(source_database_name.clone()).unwrap();
    let mut target_client = connect(target_database_name.clone()).unwrap();

    let tables_to_compare = get_tables();

    for table in tables_to_compare {
        let table_name = table.name.clone();
        let source_rows = get_rows(source_database_name.clone(), table_name.clone());
        let target_rows = get_rows(source_database_name.clone(), table_name.clone());

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

pub fn get_rows(database_name: String, table_name: String) -> Vec<postgres::Row>
{
    let mut source_client = connect(database_name.clone()).unwrap();
    let query = "SELECT * FROM ".to_string() + table_name.as_str();
    let source_rows = match source_client.query(&query, &[]) {
        Ok(rows) => rows,
        Err(err) => {
            if let Some(db_err) = err.as_db_error() {
                if db_err.code() == &SqlState::from_code("42P01") {
                    println!("Table: {} does not exist in the source database", table_name);
                    return Vec::new();
                }
            }
            panic!("Error querying source database: {:?}", err);
        }
    };
    source_rows
}

fn _row_to_string(row: &postgres::Row) -> String {
    format!("{:?}", get_cells(row))
}

/// Get cells from a row based on the column type
pub fn get_cells(row: &postgres::Row) -> Vec<String> {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            }
            "int4" => {
                let value: Option<i32> = row.try_get(name).unwrap_or(None);
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
    cells
}

pub fn get_cell_value_by_column_name(row: Row, column_name: String) -> String {
    let columns: &[Column] = row.columns();
    let cn = column_name.clone();
    let column = columns.iter().find(|column| column.name() == cn).unwrap();
    let type_ = column.type_();

    match type_.name() {
        "int8" => {
            let value: Option<i64> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(0).to_string()
        }
        "int4" => {
            let inner_row = row.clone();
            let value: Option<i32> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(0).to_string()
        }
        "varchar" => {
            let inner_row = row.clone();
            let value: Option<&str> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or("None").to_string()
        }
        "bool" => {
            let inner_row = row.clone();
            let value: Option<bool> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(false).to_string()
        }
        "timestamp" => {
            let inner_row = row.clone();
            let value: Option<SystemTime> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
        }
        _ => {
            println!("Unknown type: {:?}", type_.name());
            "Unknown".to_string()
        }
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

pub fn different_row_count() {}