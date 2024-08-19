use std::env::var;
use postgres::{Column, Row};
use crate::action::working_database::{get_cell_value_by_column_name, get_rows};

pub fn move_one_table(table_name: String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));

    // STEP 1: Get data of table from source database
    let source_rows: Vec<Row> = get_rows(source_database_name, &table_name);

    // STEP 2: Check if data has been extracted
    let target_rows: Vec<Row> = get_rows(target_database_name, &table_name);
    if target_rows.len() > 0 {
        println!("Data has been extracted from source database");
        return;
    }

    // STEP 2: Insert data into target database
    for source_row in source_rows.clone() {
        let columns: &[Column] = source_row.columns();
        let query:String = build_insert_query(&table_name, columns, &source_row);
        println!("Query: {:?}", query);
    }
}

fn build_insert_query(table_name: &String, columns: &[Column], row: &Row) -> String {
    let columns_str = columns.iter().map(|c| c.name()).collect::<Vec<_>>().join(", ");
    let values_str = columns.iter().map(
        |c|
        { format!("'{}'", get_cell_value_by_column_name(row, c.name().to_string())) }
    ).collect::<Vec<_>>().join(", ");
    format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns_str, values_str)
}