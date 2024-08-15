use std::clone::Clone;
use std::string::ToString;

pub const QUERY_UPDATE_ROW_COUNT: &str = "
            UPDATE tables
            SET row_count = ?1
            WHERE name = ?2
        ";

pub fn query_get_self_references_tables() -> &'static str {
    let query = "
        SELECT
            conname AS constraint_name,
            conrelid::regclass::varchar AS table_name,
            a.attname AS column_name
        FROM
            pg_constraint AS c
        JOIN
            pg_attribute AS a
        ON
            a.attnum = ANY(c.conkey) AND a.attrelid = c.conrelid
        WHERE
            c.confrelid = c.conrelid
            AND c.contype = 'f'
        AND c.conrelid::regclass = c.confrelid::regclass
    ";
    query
}

pub fn query_get_self_references_by_table() -> String {
    let condition = " AND table_name = ?1";
    let query = query_get_self_references_tables();
    query.to_owned() + condition
}