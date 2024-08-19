use egui::Align2;
use crate::action::move_data::move_one_table;
/// Render the menu bar

use crate::TwoDBApp;

impl TwoDBApp {
    pub fn menu_btn_migrate_data_render(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("Migrate Data", |ui|{
            if ui.button("Move One Table").clicked() {
                self.window_open = true;
            }


        });

        if self.window_open {
            egui::Window::new("Choose a table")
                .open(&mut self.window_open)
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("contents");
                    ui.horizontal(|ui| {
                        ui.label("Enter table name: ");
                        ui.text_edit_singleline(&mut self.table_name);
                    });
                    if ui.button("Move!").clicked() {
                        println!("Table name: {}", self.table_name);
                        move_one_table(self.table_name.clone());
                    }
                });
        }
    }
}
