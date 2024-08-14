use egui::Ui;
use std::env::var;
use std::thread;
use crate::TwoDBApp;
use crate::working_database::update_table_self_references;

impl TwoDBApp {
    pub fn render_update_self_referencing_tables_button(&mut self, ui: &mut Ui) {
        if ui.button("Update Self Referencing Tables").clicked() {
            self.is_busy = true;
            ui.close_menu();
            self.button_update_self_referencing_tables_event();
        }
    }

    fn button_update_self_referencing_tables_event(&mut self) {
        thread::spawn(move || {
            let database_name_source = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            update_table_self_references(&database_name_source);

            let database_name_target = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            update_table_self_references(&database_name_target);

            let text = format!("Done Get Tables for {} and {}", database_name_source, database_name_target);
            println!("{}", text);
        });
    }
}