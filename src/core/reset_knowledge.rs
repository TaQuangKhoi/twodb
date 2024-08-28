use rusqlite::{Connection, params};

pub fn reset_database() {
    let sqlite_conn = Connection::open(crate::core::SQLITE_DATABASE_PATH).unwrap();

    sqlite_conn.execute(
        "DELETE FROM tables",
        params![],
    ).unwrap();
}