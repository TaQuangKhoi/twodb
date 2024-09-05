#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TwoDBApp;
mod core;
mod database;
mod postgresql_queries;
mod app_fn_impl;
mod preparation;
mod action;
mod menu_bar;
mod state;
mod twoui;
