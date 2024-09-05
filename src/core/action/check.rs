use log::info;
use crate::core::database::pg_connect;

pub fn check_if_table_existed_in_db(database_name: &String, table_name: &String) -> bool {
    // Check if the table is existed in the target database
    let mut pg_client = pg_connect(database_name).unwrap();
    let query_check_table_existed = format!("
        SELECT EXISTS (
          SELECT 1
          FROM pg_tables
          WHERE schemaname = 'public'
            AND tablename = '{}'
        );", table_name);
    let rows = match pg_client.query(&query_check_table_existed, &[]) {
        Ok(rows) => rows,
        Err(err) => {
            info!("Error querying : {:?}", err);
            return false;
        }
    };
    let row = rows.get(0).unwrap();
    row.get(0)
}