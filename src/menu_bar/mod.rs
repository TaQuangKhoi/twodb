mod menu_btn_reset;

use std::env::var;
use std::thread;
use egui::Align2;
use log::info;
use crate::action::move_data::move_one_table;
use crate::core::get_knowledge::{get_tables_with_condition};
/// Render the menu bar

use crate::TwoDBApp;

impl TwoDBApp {
    pub fn menu_btn_migrate_data_render(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("Migrate Data", |ui| {
            if ui.button("Move One Table").clicked() {
                self.windows_state.window_move_one_table_open = true;
            }

            if ui.button("Move All Tables").clicked() {
                let is_busy = self.is_busy.clone();
                *is_busy.lock().unwrap() = true;
                let toast_text = self.toast_text.clone();

                thread::spawn(move || {
                    let source_database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));

                    let tables_from_sqlite = get_tables_with_condition(
                    " WHERE is_exported = 0"
                    );
                    info!("Tables from sqlite: {:?}", tables_from_sqlite);
                    for table in tables_from_sqlite {
                        move_one_table(table.name);
                    }

                    let text = format!("Done Move Tables for {}", source_database_name);
                    TwoDBApp::notify(text, is_busy, toast_text);
                });
            }
        });


        // Window Move One Table
        if self.windows_state.window_move_one_table_open {
            egui::Window::new("Choose a table")
                .open(&mut self.windows_state.window_move_one_table_open)

                // Center of the screen, no movement
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))

                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Enter table name: ");
                        ui.text_edit_singleline(&mut self.table_name);
                    });
                    if ui.button("Move!").clicked() {
                        info!("Table name: {}", self.table_name);
                        move_one_table(self.table_name.clone());
                    }
                    if ui.button("Build Query!").clicked() {
                        info!("Table name: {}", self.table_name);
                    }
                });
        }
    }
}
