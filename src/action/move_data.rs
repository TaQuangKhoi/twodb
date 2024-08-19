use std::any::Any;
use std::env::var;
use egui::ahash::HashMap;
use crate::action::working_database::{get_cells, get_rows};

pub fn move_one_table(table_name: String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));

    // STEP 1: Get data of table from source database
    let mut data: Vec<HashMap<String, Box<dyn Any>>> = Vec::new();
    let source_rows = get_rows(source_database_name, table_name.clone());

    let temp_row = source_rows[0].clone();
    let temp_cells = get_cells(&temp_row);
    println!("Source: {:?}", temp_cells);

    // STEP 2: Insert data into target database
}