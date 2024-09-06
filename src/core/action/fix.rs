use std::env::var;
use log::{error, info};
use postgres::Row;
use crate::core::action::TWODB_NULL;
use crate::core::action::working_database::{get_cell_value_by_column_name, get_rows};
use crate::core::database::pg_connect;
use crate::core::get_knowledge::{get_columns, get_tables_with_condition};

pub fn fix_numeric() {
    let tables_to_fix = get_tables_numeric_wrong_data(0);
    info!("Tables to fix length: {}", tables_to_fix.len());
    for table in tables_to_fix {
        fix_numeric_for_one_table(table);
    }
}

pub fn fix_numeric_for_one_table(table_name: String) {
    info!("Fixing table: {}", table_name);
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let columns_source = get_columns(&source_database_name, &table_name);
    let numeric_columns_source = columns_source.iter().filter(|c| c.data_type == "numeric").collect::<Vec<_>>();

    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    let columns_target = get_columns(&target_database_name, &table_name);
    let numeric_columns_target = columns_target.iter().filter(|c| c.data_type == "numeric").collect::<Vec<_>>();

    let source_rows: Vec<Row> = get_rows(&source_database_name, &table_name);

    let final_columns = numeric_columns_target.iter().filter(|c| {
        numeric_columns_source.iter().any(|c2| c2.name == c.name)
    }).collect::<Vec<_>>();

    let mut update_queries: Vec<String> = Vec::new();

    // Check if there is no numeric column to fix
    // let query_source;
    // let query_target;

    for row in source_rows {
        let set_pair : Vec<String> = final_columns.iter().map(|c| {
            let value = get_cell_value_by_column_name(&table_name, &row, c.name.clone());
            if value == TWODB_NULL {
                return format!("{} = NULL", c.name);
            }
            format!("{} = '{}'", c.name, value)
        }).collect();
        let id: i64 = row.get("id");
        // TODO: Build UPDATE SQL for each row
        let query = format!("UPDATE {} SET {} WHERE id = {}",
                            table_name,
                            set_pair.join(", "),
                            id.to_string());
        update_queries.push(query);
    }

    info!("Queries to update: {:?}", update_queries);
    // Execute all queries
    let mut pg_client = pg_connect(&target_database_name).unwrap();
    for query in update_queries {
        match pg_client.execute(&query, &[]) {
            Ok(_) => info!("Run query {} successfully", query),
            Err(err) => error!("Error updating: {:?}", err)
        }
    }
}

pub fn get_tables_numeric_wrong_data(limit:i8) -> Vec<String> {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let condition = format!("WHERE row_count > 0 AND \"database\" = '{}'", source_database_name);
    let tables_from_sqlite = get_tables_with_condition(
        &condition
    );
    let mut tables_to_fix = Vec::new();


    for table in tables_from_sqlite {
        let table_name = table.name.clone();
        if check_numeric_column(&table_name) {
            tables_to_fix.push(table_name);
        }
    }

    let len_tables = tables_to_fix.len();
    if (limit > len_tables as i8) || (limit == 0) {
        return tables_to_fix;
    }

    tables_to_fix[0..limit as usize].to_vec()
}

fn check_numeric_column(table_name: &String) -> bool {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let columns = get_columns(&source_database_name, table_name);
    for column in columns {
        if column.data_type == "numeric" {
            return true;
        }
    }
    false
}