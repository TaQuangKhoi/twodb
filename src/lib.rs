#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TwoDBApp;
mod working_database;
mod database;
mod table;
mod core;
mod update_clean_tables_button;
mod update_empty_tables_button;
mod widgets;
