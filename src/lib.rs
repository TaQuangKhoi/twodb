#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TwoDBApp;
mod working_database;
mod database;
mod table;
mod core;
mod widgets;
mod postgresql_queries;
mod app_fn_impl;
mod preparation;
mod sqlite_queries;
