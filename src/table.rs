use rusqlite::{Connection, params};
use crate::database::connect;
use crate::queries::{query_get_self_references_by_table, QUERY_UPDATE_ROW_COUNT};

#[derive(Debug)]
pub struct Table {
    pub id: i64,
    pub name: String,
    pub table_type: TableType,
    pub export_complexity_type: ExportComplexityType,
    pub database: String,
    pub export_order: i64,
    pub is_self_referencing: bool,
    pub self_referencing_column: String,
    pub row_count: i64, // postgres type: int8
}
impl Default for Table {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::from(""),
            table_type: TableType::BaseTable,
            export_complexity_type: ExportComplexityType::SIMPLE,
            database: String::from(""),
            export_order: 0,
            is_self_referencing: false,
            self_referencing_column: String::from(""),
            row_count: 0,
        }
    }
}
impl Table {
    pub fn increase_export_order(&mut self) {
        self.export_order += 1;
    }
    pub fn update_row_count(&mut self) {
        let query = "SELECT COUNT(*) FROM ".to_owned() + &self.name;
        let mut pg_conn = connect(self.database.clone()).unwrap();
        let rows = pg_conn.query(
            &query,
            &[],
        ).unwrap();
        let count: i64 = rows[0].get(0);
        self.row_count = count;
    }

    pub fn save_row_count_to_db(&mut self) {
        let sqlite_conn = Connection::open("twodb.db").unwrap();
        sqlite_conn.execute(
            QUERY_UPDATE_ROW_COUNT,
            params![
                self.row_count,
                self.name.clone(), // WHERE
            ],
        ).unwrap();
    }

    pub fn update_table_to_db(&mut self) {
        let query = "
            UPDATE tables
            SET

            export_order = ?1,
            row_count = ?2,
            is_self_referencing = ?3,
            self_referencing_column = ?4

            WHERE name = ?5
        ";
        let sqlite_conn = Connection::open("twodb.db").unwrap();
        sqlite_conn.execute(
            query,
            params![
                self.export_order,
                self.row_count,
                self.is_self_referencing,
                self.self_referencing_column.clone(),

                self.name.clone(), // WHERE
            ],
        ).unwrap();
    }

    /// Call to Postgres to check if the table is self-referencing
    /// Save the result to the struct
    pub fn update_self_referencing(&mut self, database_name: &String) -> bool {
        let mut client = connect(database_name.clone()).unwrap();
        // check name of self
        let rows = client.query(
            &query_get_self_references_by_table(),
            &[&self.name],
        ).unwrap();
        let result = rows.len() > 0;

        if result {
            let row = &rows[0];
            self.is_self_referencing = true;
            self.self_referencing_column = row.get(2);
        }
        result
    }

    /// Check in SQLite if the table exists
    pub fn is_table_exists(&self) -> bool {
        let sqlite_conn = Connection::open("twodb.db").unwrap();
        let mut stmt = sqlite_conn.prepare(
            "
            SELECT id
            FROM tables
            WHERE name = ?1
            AND database = ?2
            "
        ).unwrap();
        let mut rows = stmt.query(params![
            self.name,
            self.database,
        ]).unwrap();

        rows.next().unwrap_or(None).is_none().eq(&false)
    }
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
            row_count INTEGER NOT NULL
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