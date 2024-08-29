use std::env::var;

pub fn prepare_knowledge() {
    let _source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    // get_clean_tables(&source_database_name);

    let _target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    // get_clean_tables(&target_database_name);
}

