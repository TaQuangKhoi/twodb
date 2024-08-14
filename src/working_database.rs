use std::env::var;
use std::time::SystemTime;
use postgres::error::SqlState;
use rusqlite::{Connection, params};
use crate::core::get_tables;
use crate::database::connect;
use crate::table::{build_base_simple_table, create_tables_table, insert_new_table};

fn compare_database() {
    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
    let target_database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
    let mut source_client = connect(source_database_name.clone()).unwrap();
    let mut target_client = connect(target_database_name.clone()).unwrap();

    let sqlite_conn = Connection::open("twodb.db").unwrap();
    let tables_to_compare = get_tables(&sqlite_conn);

    for table in tables_to_compare {
        let table_name = table.name.clone();
        let query = "SELECT * FROM ".to_string() + table_name.as_str();
        let source_rows = match source_client.query(&query, &[]) {
            Ok(rows) => rows,
            Err(err) => {
                if let Some(db_err) = err.as_db_error() {
                    if db_err.code() == &SqlState::from_code("42P01") {
                        println!("Table: {} does not exist in the source database", table_name);
                        continue;
                    }
                }
                panic!("Error querying source database: {:?}", err);
            }
        };
        let target_rows = match target_client.query(&query, &[]) {
            Ok(rows) => rows,
            Err(err) => {
                if let Some(db_err) = err.as_db_error() {
                    if db_err.code() == &SqlState::from_code("42P01") {
                        println!("Table: {} does not exist in the target database", table_name);
                        continue;
                    }
                }
                panic!("Error querying target database: {:?}", err);
            }
        };

        let source_rows_count = source_rows.len();
        let target_rows_count = target_rows.len();
        if source_rows_count != target_rows_count {
            println!("Table: {} has different rows count: {} vs {}", table_name, source_rows_count, target_rows_count);
            continue;
        }

        for (source_row, target_row) in source_rows.iter().zip(target_rows.iter()) {
            let source_cells = get_cells(source_row);

            let target_cells = get_cells(target_row);

            if source_cells != target_cells {
                println!("Table: {} has different cells", table_name);
                println!("Source: {:?}", source_cells);
                println!("Target: {:?}", target_cells);
            }
        }
    }
}

fn _row_to_string(row: &postgres::Row) -> String {
    format!("{:?}", get_cells(row))
}

pub fn get_cells(row: &postgres::Row) -> Vec<String> {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            }
            "int4" => {
                let value: Option<i32> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(0))
            }

            "varchar" => {
                let value: Option<&str> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or("None"))
            }
            "bool" => {
                let value: Option<bool> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(false))
            }
            "timestamp" => {
                let value: Option<SystemTime> = row.try_get(name).unwrap_or(None);
                format!("{}: {}", name, value.unwrap_or(SystemTime::UNIX_EPOCH)
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())
            }
            _ => {
                println!("Unknown type: {:?}", type_.name());
                format!("{}: {}", name, "Unknown")
            }
        }
    }).collect();
    cells
}

fn get_all_tables(database_name: String) {
    let mut client = connect(database_name).unwrap();
    let query =
        "SELECT table_name, table_type \
        FROM information_schema.tables \
        WHERE table_schema = 'public'"
            .to_string();

    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    let base_tables: Vec<String> = rows.iter().map(|row| {
        let table_name: String = row.get(0);
        let table_type: String = row.get(1);
        if table_type == "BASE TABLE" {
            table_name
        } else {
            Some(String::from("")).unwrap()
        }
    }).filter(|table_name| table_name.len() > 0).collect();

    println!("Total base tables: {}", base_tables.len());

    for row in rows {
        println!("{}", _row_to_string(&row));
    }
}
/// Get all tables that do not have foreign keys
/// then save them to the tables table
pub fn get_clean_tables(database_name: &String) {
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
fn is_table_exists(conn: &Connection, table_name: String) -> bool {
    let mut stmt = conn.prepare("SELECT id FROM tables WHERE name = ?1").unwrap();
    let mut rows = stmt.query(params![table_name]).unwrap();

    rows.next().unwrap_or(None).is_none() == false
}
fn get_table_self_references(database_name: &String) {
    let mut client = connect(database_name.clone()).unwrap();
    let query = "
    SELECT \
        conname AS constraint_name, \
        conrelid::regclass AS table_name, \
        a.attname AS column_name \
    FROM \
        pg_constraint AS c \
    JOIN \
        pg_attribute AS a \
    ON \
        a.attnum = ANY(c.conkey) AND a.attrelid = c.conrelid \
    WHERE \
        c.confrelid = c.conrelid \
        AND c.contype = 'f' \
    AND c.conrelid::regclass = c.confrelid::regclass;";

    let rows = client.query(
        query,
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

fn run_database(database_name: String) {
    let mut client = match connect(database_name) {
        Ok(client) => client,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let table_name = var("TABLE_NAME").unwrap_or(String::from(""));
    let query = "SELECT * FROM ".to_string() + table_name.as_str();
    let rows = client.query(
        &query,
        &[],
    ).unwrap();

    for row in rows {
        println!("{}", _row_to_string(&row));
    }

    client.close().unwrap();
}

pub fn different_row_count() {}