use std::env::var;
use log::info;
use postgres::Row;
use postgres::error::DbError;
use crate::core::action::{check, TWODB_NULL};
use crate::core::action::working_database::{get_cell_value_by_column_name, get_rows};
use crate::core::get_knowledge::get_columns;
use crate::domain::table::Table;
use crate::domain::two_column::TwoColumn;
use crate::core::database::pg_connect;
use crate::core::table::update_is_exported;

fn set_table_is_exported(table_name: &String, is_exported: bool) {
    let mut default_table = Table::default();
    default_table.name = table_name.clone();
    default_table.is_exported = is_exported;
    update_is_exported(&mut default_table);
}

fn prepare_insert_queries(table_name: &String, rows: &Vec<Row>) -> Vec<String> {
    let mut queries: Vec<String> = Vec::new();

    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let columns_source = get_columns(&source_database_name, table_name);

    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    let columns_target = get_columns(&target_database_name, table_name);

    let final_columns = columns_target.iter().filter(|c| {
        columns_source.iter().any(|c2| c2.name == c.name)
    }).collect::<Vec<_>>();
    info!("Final columns: {:?}", final_columns);

    // STEP 2: Insert data into target database
    for source_row in rows {
        // TODO: Build columns that have in source db only

        let query: String = build_insert_query_2(table_name, &final_columns, source_row);
        queries.push(query);
    }

    queries
}

pub fn get_queries_one_table(table_name: &String) {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let source_rows: Vec<Row> = get_rows(&source_database_name, &table_name);
    let queries: Vec<String> = prepare_insert_queries(table_name, &source_rows);
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
    if target_rows.len() > 0 && source_rows.len() > 0 && source_rows.len() == target_rows.len() {
        set_table_is_exported(&table_name, true);
        info!("Data has been extracted from source database");
        return;
    }

    if !check::check_if_table_existed_in_db(&target_database_name, &table_name) {
        set_table_is_exported(&table_name, true);
        info!("Table: {} does not exist in the target database", table_name);
        return;
    }

    let queries: Vec<String> = prepare_insert_queries(&table_name, &source_rows);
    // STEP 2: Insert data into target database

    // len
    info!("Queries len: {:?}", queries.len());
    let mut failed_queries: Vec<String> = Vec::new();

    let mut pg_client = pg_connect(&target_database_name).unwrap();

    // TODO: Disable trigger before insert data

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
                // error!("Error when migrate data to table: {} \n Error: {:?}", table_name, err);

                let err: &DbError = err.as_db_error().unwrap();
                let detail = err.detail().unwrap(); // "Key (document_id)=(55) is not present in table \"materialflowresources_document\"."
                let table_ref = detail.split(" ").last().unwrap().replace("\"", "");
                let table_ref = table_ref.trim_end_matches('.').to_string();

                move_one_table(table_ref);

                // let table_name = err.table().unwrap();
                // let constraint = err.constraint().unwrap();
                // error!("Error when migrate data to table: {} by constraint: {} \n Error: {:?}", table_name, constraint, err);
                // let constraint_table = get_constraint_table(&target_database_name, constraint);
                // info!("Constraint table: {:?}", constraint_table);
            }
        };
    }

    if failed_queries.len() > 0 {
        info!("Failed queries: {:?}", failed_queries);
    }
}

fn build_insert_query_2(table_name: &String, columns: &Vec<&TwoColumn>, row: &Row) -> String {
    let columns_str = columns.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", ");
    let values_str = columns.iter().map(
        |c|
        {
            let value = get_cell_value_by_column_name(table_name, row, c.name.clone());
            if value == TWODB_NULL {
                return "NULL".to_string();
            }
            format!("'{}'", get_cell_value_by_column_name(table_name, row, c.name.clone()))
        }
    ).collect::<Vec<_>>().join(", ");
    format!("INSERT INTO {} ({}) VALUES ({})", table_name, columns_str, values_str)
}