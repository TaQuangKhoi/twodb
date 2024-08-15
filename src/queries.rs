use std::clone::Clone;
use std::string::ToString;

pub const QUERY_UPDATE_ROW_COUNT: &str = "
            UPDATE tables
            SET row_count = ?1
            WHERE name = ?2
        ";

pub const QUERY_GET_SELF_REFERENCES_TABLES: &str = "
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
        AND c.conrelid::regclass = c.confrelid::regclass;
    ";

pub const QUERY_GET_SELF_REFERENCES_BY_TABLE: &str = &*(|| {
    format!("{} WHERE table_name = ?1", QUERY_GET_SELF_REFERENCES_TABLES)
})();