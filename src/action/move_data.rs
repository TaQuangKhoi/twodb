use std::env::var;
use log::{error, info};
use postgres::{Column, Row};
use postgres::error::DbError;
use crate::action::TWODB_NULL;
use crate::action::working_database::{get_cell_value_by_column_name, get_rows};
use crate::core::get_knowledge::get_constraint_table;
use crate::core::table::Table;
use crate::database::pg_connect;

fn set_table_is_exported(table_name: &String, is_exported: bool) {
    let mut default_table = Table::default();
    default_table.name = table_name.clone();
    default_table.is_exported = is_exported;
    default_table.update_is_exported();
}

fn check_if_table_existed_in_db(database_name: &String, table_name: &String) -> bool {
    // Check if the table is existed in the target database
    let mut pg_client = pg_connect(database_name).unwrap();
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
            return false;
        }
    };
    let row = rows.get(0).unwrap();
    row.get(0)
}

fn prepare_queries(table_name: &String, rows: &Vec<Row>) -> Vec<String> {
    let mut queries: Vec<String> = Vec::new();
    // STEP 2: Insert data into target database
    for source_row in rows {
        // TODO: Build columns that have in source db only
        let columns: &[Column] = source_row.columns();

        let query: String = build_insert_query(&table_name, columns, &source_row);
        queries.push(query);
    }

    queries
}

pub fn get_queries_one_table(table_name: &String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let source_rows: Vec<Row> = get_rows(&source_database_name, &table_name);
    let queries: Vec<String> = prepare_queries(table_name, &source_rows);
    for query in queries {
        info!("Query: {:?}", query);
    }
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

    if !check_if_table_existed_in_db(&target_database_name, &table_name) {
        set_table_is_exported(&table_name, true);
        info!("Table: {} does not exist in the target database", table_name);
        return;
    }

    let queries: Vec<String> = prepare_queries(&table_name, &source_rows);
    // STEP 2: Insert data into target database

    // len
    info!("Queries len: {:?}", queries.len());
    let mut failed_queries: Vec<String> = Vec::new();

    let mut pg_client = pg_connect(&target_database_name).unwrap();

    for query in queries {
        info!("Query: {:?}", query);

        // Run query
        match pg_client.query(&query, &[]) {
            Ok(_) => {
                info!("Query executed successfully");
                set_table_is_exported(&table_name, true);
            }
            Err(err) => {
                failed_queries.push(query);
                let err: &DbError = err.as_db_error().unwrap();
                let table_name = err.table().unwrap();
                let constraint = err.constraint().unwrap();
                error!("Error when migrate data to table: {} by constraint: {} \n Error: {:?}", table_name, constraint, err);
                let constraint_table = get_constraint_table(&target_database_name, constraint);
                info!("Constraint table: {:?}", constraint_table);
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
        {
            let value = get_cell_value_by_column_name(row, c.name().to_string());
            if value == TWODB_NULL {
                return "NULL".to_string();
            }
            format!("'{}'", get_cell_value_by_column_name(row, c.name().to_string()))
        }
    ).collect::<Vec<_>>().join(", ");
    format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns_str, values_str)
}