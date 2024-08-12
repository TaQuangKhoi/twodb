use postgres::{Client, Error, NoTls};
use std::env::var;
use std::time::SystemTime;
use rusqlite::{params, Connection, Result};

fn main() {
    prepare_knowledge();

    // export based on complexity
}

#[derive(Debug)]
struct Table {
    id: i32,
    name: String,
    table_type: TableType,
    export_complexity_type: ExportComplexityType,
    database: String,
    export_order: i32,
}

#[derive(Debug)]
enum TableType {
    BaseTable,
    VIEW,
}
impl TableType {
    fn name(&self) -> &str {
        match self {
            TableType::BaseTable => "BASE TABLE",
            TableType::VIEW => "VIEW",
        }
    }
}

#[derive(Debug)]
enum ExportComplexityType {
    SIMPLE,
    COMPLEX,
}
impl ExportComplexityType {
    fn name(&self) -> &str {
        match self {
            ExportComplexityType::SIMPLE => "SIMPLE",
            ExportComplexityType::COMPLEX => "COMPLEX",
        }
    }
}

impl Table {
    fn increase_export_order(&mut self) {
        self.export_order += 1;
    }
}

fn prepare_knowledge() {
    let source_database_name = var("POSTGRES_DB_1").unwrap_or(String::from(""));
    get_clean_tables(&source_database_name);

    let target_database_name = var("POSTGRES_DB_2").unwrap_or(String::from(""));
    get_clean_tables(&target_database_name);
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
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tables (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            table_type TEXT NOT NULL,
            export_complexity_type TEXT NOT NULL,
            database TEXT NOT NULL,
            export_order INTEGER NOT NULL
        )",
        params![],
    ).unwrap();

    for row in rows {
        let table = build_base_simple_table(row.get(0), database_name.clone());

        // check if table exists
        if is_table_exists(&conn, table.name.clone()) {
            continue;
        }

        insert_new_table(&conn, table);
    }
}

fn build_base_simple_table(name: String, database: String) -> Table {
    let new_table = Table {
        id: 0,
        name,
        table_type: TableType::BaseTable,
        export_complexity_type: ExportComplexityType::SIMPLE,
        database,
        export_order: 0,
    };
    println!("{:?}", new_table);
    new_table
}

fn is_table_exists(conn: &Connection, table_name: String) -> bool {
    let mut stmt = conn.prepare("SELECT id FROM tables WHERE name = ?1").unwrap();
    let mut rows = stmt.query(params![table_name]).unwrap();

    rows.next().unwrap_or(None).is_none() == false
}

fn insert_new_table(conn: &Connection, table: Table) {
    conn.execute(
        "INSERT INTO tables (name, table_type , export_complexity_type, database, export_order)
            VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            table.name,
            table.table_type.name(),
            table.export_complexity_type.name(),
            table.database, table.export_order
        ],
    ).unwrap();
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
        println!("{}", _row_to_string(row));
    }
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
        println!("{}", _row_to_string(row));
    }

    client.close().unwrap();
}

fn _row_to_string(row: postgres::Row) -> String {
    let columns = row.columns();
    let cells: Vec<String> = columns.iter().map(|column| {
        let name = column.name();
        let type_ = column.type_();
        match type_.name() {
            "int8" => {
                let value: Option<i64> = row.try_get(name).unwrap_or(None);
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

    format!("{:?}", cells)
}

/**
 * Connect to the database
 *
 * Author : Ta Quang Khoi
 */
fn connect(database_name: String) -> Result<Client, Error> {
    let username = var("POSTGRES_USER").unwrap_or(String::from(""));
    let password = var("POSTGRES_PASSWORD").unwrap_or(String::from(""));
    let host = var("POSTGRES_HOST").unwrap_or(String::from(""));

    let database_url = format!("postgresql://{}:{}@{}/{}", username, password, host, database_name);

    let client = Client::connect(
        database_url.as_str(),
        NoTls,
    )?;

    Ok(client)
}
