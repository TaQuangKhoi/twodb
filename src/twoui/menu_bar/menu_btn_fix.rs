use crate::TwoDBApp;

impl TwoDBApp {
    pub fn menu_btn_fix_render(&mut self, _: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("Fix", |ui| {
            if ui.button("Fix Numeric Data").clicked() {

            }
        });
    }
}