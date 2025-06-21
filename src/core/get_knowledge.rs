use log::{debug, error, info};
use rusqlite::{Connection, params};
use crate::core::SQLITE_DATABASE_PATH;
use crate::core::database::pg_connect;
use crate::domain::table::{Table, TableType, ExportComplexityType};
use crate::domain::two_column::TwoColumn;

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
        crate::core::table::update_row_count(&mut inner_table); // Long-running query
        result.push(inner_table);
    }
    result
}

pub fn get_constraint_table(database_name: &String, constraint: &str) -> String {
    let mut pg_client = pg_connect(database_name).unwrap();
    let query = format!("
    SELECT
        conname AS constraint_name,
        conrelid::regclass AS table_name,
        confrelid::regclass::text AS referenced_table_name
    FROM
        pg_constraint
    WHERE
        contype = 'f'
        AND conname = '{}';", constraint);
    info!("Query: {}", query);

    match pg_client.query (&query, &[]) {
        Ok(rows) => {
            debug!("Rows: {:?}", rows);
            let row = rows.get(0).unwrap();
            let table_name: String = row.get("referenced_table_name");
            table_name
        },
        Err(err) => {
            error!("Error: {:?}", err);
            String::from("")
        }
    }
}

pub fn get_columns(database_name: &String, table_name: &String) -> Vec<TwoColumn> {
    let mut pg_client = pg_connect(database_name).unwrap();
    let query = format!("
    SELECT
        column_name,
        data_type,
        character_maximum_length,
        numeric_precision,
        numeric_scale,
        is_nullable
    FROM
        information_schema.columns
    WHERE
        table_name = '{}';", table_name);

    let mut columns = Vec::new();

    match pg_client.query (&query, &[]) {
        Ok(rows) => {
            for row in rows {
                let column: TwoColumn = TwoColumn {
                    name: row.get("column_name"),
                    data_type: row.get("data_type"),
                };
                let _is_nullable: String = row.get("is_nullable");
                &columns.push(column);
            }
            columns
        },
        Err(err) => {
            error!("Error: {:?}", err);
            columns
        }
    }
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

pub fn get_tables_with_condition(condition: &str) -> Vec<Table>
{
    let sqlite_conn = Connection::open(SQLITE_DATABASE_PATH).unwrap();
    let query = String::from(SELECT_PART) + " " + condition;
    let mut stmt = sqlite_conn.prepare(&*query).unwrap();

    // println!("Query: {}", query);
    let tables_iter = stmt.query_map([], |row| {
        // println!("Row: {:?}", row);
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
