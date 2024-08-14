/*! This file contains the core logic of the application. */

use rusqlite::{Connection, params};
use crate::table::{ExportComplexityType, Table, TableType};

/// Get all tables from the SQLite database
pub fn get_tables(conn: &Connection) -> Vec<Table> {
    let mut stmt = conn.prepare(
        "SELECT id, name, table_type, export_complexity_type, database, export_order FROM tables"
    ).unwrap();
    let tables = stmt.query_map(params![], |row| {
        Ok(Table {
            id: row.get(0)?,
            name: row.get(1)?,
            table_type: TableType::BaseTable,
            export_complexity_type: ExportComplexityType::SIMPLE,
            database: row.get(4)?,
            export_order: row.get(5)?,
            is_self_referencing: false,
            row_count: 0,
        })
    }).unwrap();
    let mut result = Vec::new();
    for table in tables {
        let mut inner_table = table.unwrap();
        inner_table.update_row_count();
        result.push(inner_table);
    }
    result
}