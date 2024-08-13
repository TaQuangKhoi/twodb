use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct Table {
    pub id: i32,
    pub name: String,
    pub table_type: TableType,
    pub export_complexity_type: ExportComplexityType,
    pub database: String,
    pub export_order: i32,
    pub is_self_referencing: bool,
}
impl Table {
    pub fn increase_export_order(&mut self) {
        self.export_order += 1;
    }
}
pub fn create_tables_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tables (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            table_type TEXT NOT NULL,
            export_complexity_type TEXT NOT NULL,
            database TEXT NOT NULL,
            export_order INTEGER NOT NULL,
            is_self_referencing BOOLEAN NOT NULL
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
    };
    new_table
}
pub fn build_self_references_table(name: String, database: String) -> Table{
    let new_table = Table {
        id: 0,
        name,
        table_type: TableType::BaseTable,
        export_complexity_type: ExportComplexityType::COMPLEX,
        database,
        export_order: 0,
        is_self_referencing: true,
    };
    new_table
}
pub fn insert_new_table(conn: &Connection, table: Table) {
    conn.execute(
        "INSERT INTO tables (name,
        table_type, export_complexity_type, database,
        export_order,
        is_self_referencing
        )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            table.name,
            table.table_type.name(),
            table.export_complexity_type.name(),
            table.database,
            table.export_order,
            table.is_self_referencing,
        ],
    ).unwrap();
}

pub const BASE_TABLE_STR: &str = "BASE TABLE";

#[derive(Debug)]
pub enum TableType {
    BaseTable,
    VIEW,
}
impl TableType {
    pub fn name(&self) -> &str {
        match self {
            TableType::BaseTable => BASE_TABLE_STR,
            TableType::VIEW => "VIEW",
        }
    }
}

#[derive(Debug)]
pub enum ExportComplexityType {
    SIMPLE,
    COMPLEX,
}
impl ExportComplexityType {
    pub fn name(&self) -> &str {
        match self {
            ExportComplexityType::SIMPLE => "SIMPLE",
            ExportComplexityType::COMPLEX => "COMPLEX",
        }
    }
}