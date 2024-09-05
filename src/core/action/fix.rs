
pub fn get_tables_numeric_wrong_data() -> Vec<String> {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let condition = format!("WHERE \"database\" = '{}'", source_database_name);
    let tables_from_sqlite = get_tables_with_condition(
        &condition
    );
    let mut tables_to_fix = Vec::new();
    for table in tables_from_sqlite {
        let columns = get_columns(&source_database_name, &table.name);
        for column in columns {
            if column.data_type == "numeric" {
                let table_name: String = table.get_table_name_as_str();
                tables_to_fix.push(table_name);
                break;
            }
        }
    }

    tables_to_fix
}