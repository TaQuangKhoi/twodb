use std::env::var;
use std::time::SystemTime;
use chrono::NaiveDate;
use log::{error, info};
use postgres::error::SqlState;
use postgres::{Column, Row};
use crate::core::action::TWODB_NULL;
use crate::core::database::pg_connect;

pub fn get_rows(database_name: &String, table_name: &String) -> Vec<Row> {
    // TODO: Check if table empty (no records)

    let mut pg_client = pg_connect(database_name).unwrap();
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

/// Get cells from a row based on the column type
pub fn get_cells(row: &Row) -> Vec<String> {
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
            "date" => {
                let value: Option<NaiveDate> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()))
            }
            _ => {
                error!("Unknown type: {:?}", type_.name());
                format!("{}: {}", name, "Unknown")
            }
        }
    }).collect();
    cells
}

pub fn get_cell_value_by_column_name(table_name: &String, row: &Row, column_name: String) -> String {
    let columns: &[Column] = row.columns();
    let column = columns.iter().find(|column| column.name() == column_name.clone()).unwrap();
    let type_ = column.type_();

    match type_.name() {
        "int8" => {
            let value: Option<i64> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or(0).to_string()
        }
        "int4" => {
            let value: Option<i32> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or(0).to_string()
        }
        "varchar" => {
            let value: Option<&str> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or("None").to_string()
        }
        "bool" => {
            let value: Option<bool> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or(false).to_string()
        }
        "timestamp" => {
            let value: Option<SystemTime> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            use chrono::prelude::{DateTime, Utc};
            let dt: DateTime<Utc> = value.clone().unwrap().into();
            dt.to_rfc3339()
        }

        // This datatype is not supported by crate postgres
        "numeric" => {
            // Code chữa cháy, code này chạy cực lâu
            let column_name = column.name();
            let row_id: i64 = row.try_get("id").unwrap_or(0);
            let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            let mut pg_client = pg_connect(&source_database_name).unwrap();
            let query = format!("SELECT {}::text FROM {} WHERE id = {}", column_name, table_name, row_id);
            let rows = pg_client.query(&query, &[]).unwrap();
            let row = rows.get(0).unwrap();
            let value: Option<&str> = row.try_get(0).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or("0.0").to_string()
        }

        "text" => {
            let value: Option<&str> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or("None").to_string()
        }
        "date" => {
            let value: Option<NaiveDate> = row.try_get(column.name()).unwrap_or(None);
            if value.is_none() {
                return TWODB_NULL.to_string();
            }
            value.unwrap_or(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).to_string()
        }
        _ => {
            error!("get_cell_value_by_column_name - Unknown type: {:?}", type_.name());
            "Unknown".to_string()
        }
    }
}

pub fn different_row_count() {}