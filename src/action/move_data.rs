use std::env::var;
use log::{error, info};
use postgres::{Column, Row};
use crate::action::working_database::{get_cell_value_by_column_name, get_rows};
use crate::core::table::Table;
use crate::database::connect;

fn set_table_is_exported(table_name: &String, is_exported: bool) {
    let mut default_table = Table::default();
    default_table.name = table_name.clone();
    default_table.is_exported = is_exported;
    default_table.update_is_exported();
}

pub fn move_one_table(table_name: String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));

    // STEP 1: Get data of table from source database and target database
    let source_rows: Vec<Row> = get_rows(&source_database_name, &table_name);
    let target_rows: Vec<Row> = get_rows(&target_database_name, &table_name);


    // STEP 2: Check if data has been extracted

    // Case : Both source and target databases are empty
    if source_rows.len() == 0 && target_rows.len() == 0 {
        set_table_is_exported(&table_name, true);
        info!("Both source and target databases are empty");
        return;
    }

    // Case 1: Data has been extracted
    if target_rows.len() > 0 {
        set_table_is_exported(&table_name, true);
        info!("Data has been extracted from source database");
        return;
    }

    // Check if the table is existed in the target database
    let mut pg_client = connect(target_database_name.clone()).unwrap();
    let query_check_table_existed = format!("
        SELECT EXISTS (
          SELECT 1
          FROM pg_tables
          WHERE schemaname = 'public'
            AND tablename = '{}'
        );", table_name);
    let rows = match pg_client.query(&query_check_table_existed, &[]) {
        Ok(rows) => rows,
        Err(err) => {
            info!("Error querying : {:?}", err);
            return;
        }
    };
    let row = rows.get(0).unwrap();
    let is_table_existed: bool = row.get(0);
    if !is_table_existed {
        set_table_is_exported(&table_name, true);
        info!("Table: {} does not exist in the target database", table_name);
        return;
    }


    let mut queries: Vec<String> = Vec::new();
    // STEP 2: Insert data into target database
    for source_row in source_rows.clone() {
        let columns: &[Column] = source_row.columns();
        let query: String = build_insert_query(&table_name, columns, &source_row);
        queries.push(query);
    }

    // len
    info!("Queries len: {:?}", queries.len());
    let mut failed_queries: Vec<String> = Vec::new();
    for query in queries {
        let mut pg_client = connect(target_database_name.clone()).unwrap();
        info!("Query: {:?}", query);
        match pg_client.query(&query, &[]) {
            Ok(_) => {
                info!("Query executed successfully");
            }
            Err(err) => {
                failed_queries.push(query);
                error!("Error when insert : {:?}", err);
            }
        };
    }

    if failed_queries.len() > 0 {
        info!("Failed queries: {:?}", failed_queries);
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