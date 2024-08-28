use egui::TextBuffer;
use rusqlite::{Connection, MappedRows, params, Row};
use crate::core::SQLITE_DATABASE_PATH;
use crate::core::table::{ExportComplexityType, Table, TableType};

const SELECT_PART: &str = "SELECT
            id,
            name,
            table_type,
            export_complexity_type,
            database,
            export_order,
            is_self_referencing,
            self_referencing_column,
            row_count,
            is_exported
        FROM tables ";

/// Get all tables from the SQLite database
/// Issue: Long running query
pub fn get_tables() -> Vec<Table> {
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    let mut stmt = sqlite_conn.prepare(
        SELECT_PART
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
            self_referencing_column: String::from(""),
            row_count: row.get(8)?,
            is_exported: false,
        })
    }).unwrap();
    let mut result = Vec::new();
    for table in tables {
        let mut inner_table = table.unwrap();
        inner_table.update_row_count(); // Long-running query
        result.push(inner_table);
    }
    result
}

pub fn get_tables_of_database(database_name: &String) -> Vec<Table>
{
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    let query = String::from(SELECT_PART) + " WHERE database = ?1";
    let mut stmt = sqlite_conn.prepare(
        &*query
    ).unwrap();

    let tables_iter = stmt.query_map([database_name], |row| {
        Ok(Table {
            id: row.get(0)?,
            name: row.get(1)?,
            table_type: TableType::BaseTable,
            export_complexity_type: ExportComplexityType::SIMPLE,
            database: row.get(4)?,
            export_order: row.get(5)?,
            is_self_referencing: false,
            self_referencing_column: String::from(""),
            row_count: row.get(8)?,
            is_exported: false,
        })
    }).unwrap();
    let mut result = Vec::new();
    for table in tables_iter {
        result.push(table.unwrap());
    }
    result
}

pub fn get_tables_with_condition(database_name: &String, condition: &str) -> Vec<Table>
{
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    let query = String::from(SELECT_PART) + condition;
    let mut stmt = sqlite_conn.prepare(&*query).unwrap();

    println!("Query: {}", query);
    let tables_iter = stmt.query_map([], |row| {
        println!("Row: {:?}", row);
        Ok(Table {
            id: row.get(0)?,
            name: row.get(1)?,
            table_type: TableType::BaseTable,
            export_complexity_type: ExportComplexityType::SIMPLE,
            database: row.get(4)?,
            export_order: row.get(5)?,
            is_self_referencing: false,
            self_referencing_column: String::from(""),
            row_count: row.get("row_count")?,
            is_exported: false,
        })
    }).expect("Error in get_tables_with_condition");
    let mut result = Vec::new();
    for table in tables_iter {
        result.push(table.unwrap());
    }
    result
}