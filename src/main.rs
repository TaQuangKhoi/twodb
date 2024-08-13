pub mod database;
mod preparation;
mod table;
mod core;
mod working_database;

use database::connect;
use std::env::var;
use postgres::error::SqlState;
use rusqlite::{Connection};
use crate::core::get_tables;
use crate::preparation::prepare_knowledge;
use crate::working_database::get_cells;

fn main() {
    prepare_knowledge();
    compare_database();
    // export based on complexity
}

