#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TwoDBApp;
mod working_database;
mod database;
mod table;
mod core;
