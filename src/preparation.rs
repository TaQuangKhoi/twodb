use std::env::var;
use rusqlite::{Connection, params};
use crate::database::connect;
use crate::table::{build_base_simple_table, create_tables_table, insert_new_table};

pub fn prepare_knowledge() {
    let source_database_name = var("POSTGRES_DB_1").unwrap_or(String::from(""));
    get_clean_tables(&source_database_name);

    let target_database_name = var("POSTGRES_DB_2").unwrap_or(String::from(""));
    get_clean_tables(&target_database_name);
}

fn is_table_exists(conn: &Connection, table_name: String) -> bool {
    let mut stmt = conn.prepare("SELECT id FROM tables WHERE name = ?1").unwrap();
    let mut rows = stmt.query(params![table_name]).unwrap();

    rows.next().unwrap_or(None).is_none() == false
}

fn get_clean_tables(database_name: &String) {
    let mut client = connect(database_name.clone()).unwrap();
    let query = "SELECT table_name \
        FROM information_schema.tables \
        WHERE table_schema = 'public' \
        AND table_type = 'BASE TABLE' \
        AND table_name NOT IN ( \
            SELECT DISTINCT table_name \
            FROM information_schema.table_constraints \
            WHERE constraint_type = 'FOREIGN KEY' \
            AND table_schema = 'public' \
        );".to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    let conn = Connection::open("twodb.db").unwrap();
    create_tables_table(&conn);

    for row in rows {
        let table = build_base_simple_table(row.get(0), database_name.clone());

        // check if table exists
        if is_table_exists(&conn, table.name.clone()) {
            continue;
        }

        insert_new_table(&conn, table);
    }
    conn.close().unwrap();
}