use std::env::var;
use std::thread;
use crate::TwoDBApp;
use crate::working_database::{update_all_tables};

impl TwoDBApp {
    pub fn render_update_tables_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Update Tables").clicked() {
            self.is_busy_old = true;
            ui.close_menu();
            self.button_update_tables_event();
        }
    }

    fn button_update_tables_event(&mut self) {
        thread::spawn(move || {
            let database_name_source = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            update_all_tables(&database_name_source);

            let database_name_target = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            update_all_tables(&database_name_target);

            let text = format!("Done Get All Tables for {} and {}", database_name_source, database_name_target);
        });
    }
}