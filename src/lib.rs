#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TwoDBApp;
mod core;
mod app_fn_impl;
mod action;
mod state;
mod twoui;
