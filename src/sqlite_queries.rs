/// SQL dialect: SQLite
pub fn query_update_row_count() -> &'static str {
    "
        UPDATE tables
        SET row_count = ?1
        WHERE name = ?2
    "
}