use std::any::Any;
use std::env::var;
use std::collections::HashMap;
use crate::action::working_database::{get_cells, get_rows};

pub fn move_one_table(table_name: String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));

    // STEP 1: Get data of table from source database
    let mut data: Vec<HashMap<String, Box<dyn Any>>> = Vec::new();
    let source_rows = get_rows(source_database_name, table_name.clone());
    for source_row in &source_rows {
        let columns = source_row.columns();
        let cells = get_cells(&source_row);
        let mut row: HashMap<String, Box<dyn Any>> = HashMap::new();
        for (i, cell) in cells.iter().enumerate() {
            let column_name = columns[i].name();
            println!("Column name: {}", column_name);
            row.insert(column_name.parse().unwrap(), Box::new(cell.clone()));
        }
        data.push(row);
    }

    // STEP 2: Insert data into target database
    for row in data {
        for (column_name, cell) in row {
            let value = cell.downcast_ref::<String>().unwrap();
            println!("{}", value);
        }
    }
}