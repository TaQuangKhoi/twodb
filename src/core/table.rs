use rusqlite::{Connection, params};
use crate::core::database::pg_connect;
use crate::core::postgresql_queries::query_get_self_references_by_table;
use crate::core::sqlite_queries::query_update_row_count;
use crate::domain::table::{Table, TableType, ExportComplexityType};

const SQLITE_DATABASE_PATH: &str = "twodb.db";

/// Get postgres row count, then update the struct and SQLite
pub fn update_row_count(table: &mut Table) {
    let pg_query = "SELECT COUNT(*) FROM ".to_owned() + &table.name;
    let mut pg_conn = pg_connect(&table.database).unwrap();
    let rows = pg_conn.query(
        &pg_query,
        &[],
    ).unwrap();
    let count: i64 = rows[0].get(0);
    table.row_count = count;
}

pub fn save_row_count_to_db(table: &Table) {
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    sqlite_conn.execute(
        query_update_row_count(),
        params![
            table.row_count,
            table.name.clone(), // WHERE
        ],
    ).unwrap();
}

/// Update the table in SQLite
pub fn update_table_to_db(table: &Table) {
    let query = "
        UPDATE tables
        SET

        export_order = ?1,
        row_count = ?2,
        is_self_referencing = ?3,
        self_referencing_column = ?4

        WHERE name = ?5 AND database = ?6
    ";
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    sqlite_conn.execute(
        query,
        params![
            table.export_order,
            table.row_count,
            table.is_self_referencing,
            table.self_referencing_column.clone(),

            // WHERE
            table.name.clone(),
            table.database.clone(),
        ],
    ).unwrap();
}

/// Call to Postgres to check if the table is self-referencing
/// Save the result to the struct
pub fn update_self_referencing(table: &mut Table, database_name: &String) -> bool {
    let mut client = pg_connect(database_name).unwrap();
    // check name of self
    let rows = client.query(
        &query_get_self_references_by_table(),
        &[&table.name],
    ).unwrap();
    let result = rows.len() > 0;

    if result {
        let row = &rows[0];
        table.is_self_referencing = true;
        table.self_referencing_column = row.get(2);
    }
    result
}

pub fn update_is_exported(table: &mut Table) {
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    sqlite_conn.execute(
        "
        UPDATE tables
        SET is_exported = ?1
        WHERE name = ?2
        ",
        params![
            table.is_exported,

            table.name.clone(),
        ],
    ).unwrap();
}

/// Check in SQLite if the table exists
pub fn is_table_exists(table: &Table) -> bool {
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    let mut stmt = sqlite_conn.prepare(
        "
        SELECT id
        FROM tables
        WHERE name = ?1
        AND database = ?2
        "
    ).unwrap();
    let mut rows = stmt.query(params![
        table.name,
        table.database,
    ]).unwrap();

    rows.next().unwrap_or(None).is_none().eq(&false)
}

/// Create a new table named `tables` in the SQLite database
pub fn create_tables_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tables (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            table_type TEXT NOT NULL,
            export_complexity_type TEXT NOT NULL,
            database TEXT NOT NULL,
            export_order INTEGER NOT NULL,
            is_self_referencing BOOLEAN NOT NULL,
            self_referencing_column TEXT,
            row_count INTEGER NOT NULL DEFAULT 0,
            is_exported BOOLEAN NOT NULL DEFAULT FALSE
        )",
        params![],
    ).unwrap();
}

pub fn build_base_simple_table(name: String, database: String) -> Table {
    let new_table = Table {
        id: 0,
        name,
        table_type: TableType::BaseTable,
        export_complexity_type: ExportComplexityType::SIMPLE,
        database,
        export_order: 0,
        is_self_referencing: false,
        self_referencing_column: String::from(""),
        row_count: 0,
        is_exported: false,
    };
    new_table
}

pub fn build_self_references_table(name: String, database: String) -> Table {
    let new_table = Table {
        id: 0,
        name,
        table_type: TableType::BaseTable,
        export_complexity_type: ExportComplexityType::COMPLEX,
        database,
        export_order: 0,
        is_self_referencing: true,
        self_referencing_column: String::from(""),
        row_count: 0,
        is_exported: false,
    };
    new_table
}

pub fn insert_new_table(conn: &Connection, table: Table) {
    conn.execute(
        "INSERT INTO tables (name,
        table_type, export_complexity_type, database,
        export_order,
        is_self_referencing,
        self_referencing_column,
        row_count
        )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6,
            ?7,
            ?8
            )",
        params![
            table.name,
            table.table_type.name(),
            table.export_complexity_type.name(),
            table.database,
            table.export_order,
            table.is_self_referencing,
            table.self_referencing_column,
            table.row_count,
        ],
    ).unwrap();
}