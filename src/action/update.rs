/// Update knowledge base about target and source databases

use rusqlite::Connection;
use crate::core::table::{build_base_simple_table, create_tables_table, insert_new_table, Table};
use crate::database::pg_connect;
use crate::postgresql_queries::query_get_self_references_tables;

const SQLITE_DATABASE_PATH: &str = "twodb.db";

pub fn update_table_self_references(database_name: &String) {
    let mut client = pg_connect(&database_name).unwrap();
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

pub fn update_empty_tables(database_name: &String) {
    let mut pg_client = pg_connect(database_name).unwrap();
    let query =
        "SELECT n.nspname,
            c.relname AS table_name,
            c.reltuples
        FROM pg_class c
        INNER JOIN pg_namespace n ON (n.oid = c.relnamespace)
        WHERE c.reltuples = 0 AND c.relkind = 'r'
        and n.nspname = 'public';";

    let rows = pg_client.query(
        query,
        &[],
    ).unwrap();

    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    create_tables_table(&sqlite_conn);

    for row in rows {
        let table_name: String = row.get(1);
        let mut table: Table = build_base_simple_table(table_name, database_name.clone());

        // check if table exists, update row count
        if table.is_table_exists() {
            table.update_row_count();
            continue;
        }

        insert_new_table(&sqlite_conn, table);
    }
}

/// Get all tables that do not have foreign keys
/// then save them to the tables table
pub fn update_clean_tables(database_name: &String) {
    let mut client = pg_connect(database_name).unwrap();
    let query = "SELECT table_name
        FROM information_schema.tables
        WHERE table_schema = 'public'
        AND table_type = 'BASE TABLE'
        AND table_name NOT IN (
            SELECT DISTINCT table_name
            FROM information_schema.table_constraints
            WHERE constraint_type = 'FOREIGN KEY'
            AND table_schema = 'public'
        );".to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    let conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    create_tables_table(&conn);

    for row in rows {
        let table_name: String = row.get(0);
        let mut table = build_base_simple_table(table_name, database_name.clone());
        table.update_row_count();
        // check if table exists
        if table.is_table_exists() {
            continue;
        }

        insert_new_table(&conn, table);
    }
}

pub fn update_all_tables(database_name: &String) {
    let mut client = pg_connect(database_name).unwrap();
    let query = "
        SELECT table_name, table_type
        FROM information_schema.tables
        WHERE table_schema = 'public'
        AND table_type = 'BASE TABLE'
        ".to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    create_tables_table(&sqlite_conn);

    for row in rows {
        let table_name: String = row.get(0);

        let mut table: Table = build_base_simple_table(table_name.clone(), database_name.clone());
        table.update_self_referencing(database_name);
        table.update_row_count();

        // check if table exists
        if table.is_table_exists() {
            table.update_table_to_db();
            continue;
        }

        insert_new_table(&sqlite_conn, table);
    }
}