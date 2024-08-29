use std::env::var;
use std::time::SystemTime;
use log::{error, info};
use postgres::error::SqlState;
use postgres::{Column, Row};
use crate::core::get_knowledge::get_tables;
use crate::database::connect;

/// Compare two databases (PostgreSQL)
fn compare_database() {
    let source_pg_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_pg_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    let mut source_pg_client = connect(source_pg_database_name.clone()).unwrap();
    let mut target_pg_client = connect(target_pg_database_name.clone()).unwrap();

    let tables_to_compare = get_tables();

    for table in tables_to_compare {
        let table_name = table.name.clone();
        let source_rows = get_rows(&source_pg_database_name, &table_name);
        let target_rows = get_rows(&source_pg_database_name, &table_name);

        let source_rows_count = source_rows.len();
        let target_rows_count = target_rows.len();
        if source_rows_count != target_rows_count {
            info!("Table: {} has different rows count: {} vs {}", table_name, source_rows_count, target_rows_count);
            continue;
        }

        for (source_row, target_row) in source_rows.iter().zip(target_rows.iter()) {
            let source_cells = get_cells(source_row);

            let target_cells = get_cells(target_row);

            if source_cells != target_cells {
                info!("Table: {} has different cells", table_name);
                info!("Source: {:?}", source_cells);
                info!("Target: {:?}", target_cells);
            }
        }
    }
}

pub fn get_rows(database_name: &String, table_name: &String) -> Vec<postgres::Row>
{
    let mut pg_client = connect(database_name.clone()).unwrap();
    let query = "SELECT * FROM ".to_string() + table_name.as_str();
    let rows = match pg_client.query(&query, &[]) {
        Ok(rows) => rows,
        Err(err) => {
            if let Some(db_err) = err.as_db_error() {
                if db_err.code() == &SqlState::from_code("42P01") {
                    info!("Table: {} does not exist in the database {}", table_name, database_name);
                    return Vec::new();
                }
            }
            panic!("Error querying database {}: {:?}", database_name, err);
        }
    };
    rows
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
                error!("Unknown type: {:?}", type_.name());
                format!("{}: {}", name, "Unknown")
            }
        }
    }).collect();
    cells
}

pub fn get_cell_value_by_column_name(row: &Row, column_name: String) -> String {
    let columns: &[Column] = row.columns();
    let column = columns.iter().find(|column| column.name() == column_name.clone()).unwrap();
    let type_ = column.type_();
    match type_.name() {
        "int8" => {
            let value: Option<i64> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(0).to_string()
        }
        "int4" => {
            let value: Option<i32> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(0).to_string()
        }
        "varchar" => {
            let value: Option<&str> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or("None").to_string()
        }
        "bool" => {
            let value: Option<bool> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(false).to_string()
        }
        "timestamp" => {
            let value: Option<SystemTime> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string()
        }
        "numeric" => {
            let value: Option<f64> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or(0.0).to_string()
        }
        "text" => {
            let value: Option<&str> = row.try_get(column.name()).unwrap_or(None);
            value.unwrap_or("None").to_string()
        }
        _ => {
            error!("get_cell_value_by_column_name - Unknown type: {:?}", type_.name());
            "Unknown".to_string()
        }
    }
}

fn run_database(database_name: String) {
    let mut client = match connect(database_name) {
        Ok(client) => client,
        Err(err) => {
            error!("Error: {}", err);
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
        info!("{}", _row_to_string(&row));
    }

    client.close().unwrap();
}

pub fn different_row_count() {}