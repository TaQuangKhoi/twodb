use egui::Align2;
use log::info;
use crate::TwoDBApp;

impl TwoDBApp {
    pub fn menu_btn_reset_render(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("Reset", |ui| {
            if ui.button("Reset").clicked() {
                self.windows_state.window_reset_open = true;
            }
        });

        /// Window Reset
        if self.windows_state.window_reset_open {
            // let mut self_clone : *mut TwoDBApp = self;
            egui::Window::new("Reset")
                .open(&mut self.windows_state.window_reset_open)

                // Center of the screen, no movement
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))

                .show(ctx, |ui| {
                    ui.label("Are you sure you want to reset?");
                    ui.horizontal(|ui| unsafe {
                        if ui.button("Yes").clicked() {
                            // Delete Database in SQLite
                            info!("Resetting database");
                            // (*self_clone).windows_state.window_reset_open = false;
                        }
                        if ui.button("No").clicked() {
                            info!("Cancel Resetting database");
                        }
                    });
                });
        }
    }
}