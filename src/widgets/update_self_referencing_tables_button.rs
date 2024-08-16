use egui::Ui;
use std::env::var;
use std::thread;
use crate::TwoDBApp;
use crate::action::update::update_table_self_references;

impl TwoDBApp {
    pub fn render_update_self_referencing_tables_button(&mut self, ui: &mut Ui) {
        if ui.button("Update Self Referencing Tables").clicked() {
            ui.close_menu();
            self.button_update_self_referencing_tables_event();
        }
    }

    fn button_update_self_referencing_tables_event(&mut self) {
        let is_busy = self.is_busy.clone();
        *is_busy.lock().unwrap() = true;
        let toast_text = self.toast_text.clone();

        thread::spawn(move || {
            let database_name_source = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            update_table_self_references(&database_name_source);

            let database_name_target = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            update_table_self_references(&database_name_target);

            let text = format!("Done Get Tables for {} and {}", database_name_source, database_name_target);
            TwoDBApp::notify(text, is_busy, toast_text);
        });
    }
}