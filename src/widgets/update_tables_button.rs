use std::env::var;
use std::thread;
use crate::TwoDBApp;
use crate::working_database::{update_all_tables};

impl TwoDBApp {
    pub fn render_update_tables_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Update Tables").clicked() {
            ui.close_menu();
            self.button_update_tables_event();
        }
    }

    fn button_update_tables_event(&mut self) {
        let is_busy = self.is_busy.clone();
        *is_busy.lock().unwrap() = true;
        let toast_text = self.toast_text.clone();

        thread::spawn(move || {
            let database_name_source = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            update_all_tables(&database_name_source);

            let database_name_target = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            update_all_tables(&database_name_target);

            let text = format!("Done Get All Tables for {} and {}", database_name_source, database_name_target);
            TwoDBApp::notify(text, is_busy, toast_text);
        });
    }
}