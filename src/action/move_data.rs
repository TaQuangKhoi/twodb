use std::any::Any;
use std::env::var;
use std::collections::HashMap;
use postgres::Row;
use crate::action::working_database::{get_cell_value_by_column_name, get_rows};

pub fn move_one_table(table_name: String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));

    // STEP 1: Get data of table from source database
    let source_rows: Vec<Row> = get_rows(source_database_name, table_name.clone());

    // STEP 2: Insert data into target database
    for source_row in source_rows.clone() {
        let columns = source_row.columns();
        let id: String = get_cell_value_by_column_name(source_row, "number".to_string());
        println!("ID: {:?}", id);
    }
}