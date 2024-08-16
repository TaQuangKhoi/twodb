/// Update knowledge base about target and source databases
///
use rusqlite::Connection;
use crate::core::table::{build_base_simple_table, create_tables_table, insert_new_table, Table};
use crate::database::connect;
use crate::postgresql_queries::query_get_self_references_tables;

const SQLITE_DATABASE_PATH: &str = "twodb.db";

pub fn update_table_self_references(database_name: &String) {
    let mut client = connect(database_name.clone()).unwrap();
    let query = query_get_self_references_tables();

    let rows = client.query(
        query,
        &[],
    ).unwrap();

    let conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    create_tables_table(&conn);

    for row in rows {
        let table_name: String = row.get(1);
        let self_referencing_column: String = row.get(2);

        let mut table: Table = build_base_simple_table(table_name.clone(), database_name.clone());
        table.is_self_referencing = true;
        table.self_referencing_column = self_referencing_column;

        // check if table exists
        if table.is_table_exists() {
            table.update_table_to_db();
            continue;
        }

        insert_new_table(&conn, table);
    }
}