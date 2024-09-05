/*! This file contains the core logic of the application. */

pub mod get_knowledge;
pub mod table;
mod sqlite_queries;
pub(crate) mod reset_knowledge;
pub mod database;

pub const SQLITE_DATABASE_PATH: &str = "twodb.db";

#[derive(Debug)]
pub struct TwoColumn {
    pub name: String,
    pub data_type: String,
}