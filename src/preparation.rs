use std::env::var;
use crate::working_database::get_clean_tables;

pub fn prepare_knowledge() {
    let source_database_name = var("POSTGRES_DB_1").unwrap_or(String::from(""));
    get_clean_tables(&source_database_name);

    let target_database_name = var("POSTGRES_DB_2").unwrap_or(String::from(""));
    get_clean_tables(&target_database_name);
}

