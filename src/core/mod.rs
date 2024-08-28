/*! This file contains the core logic of the application. */

pub mod get_knowledge;
pub mod table;
mod sqlite_queries;
pub(crate) mod reset_knowledge;

pub const SQLITE_DATABASE_PATH: &str = "twodb.db";