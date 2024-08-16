///
///
use crate::TwoDBApp;

impl TwoDBApp {
    pub fn menu_btn_migrate_data_render(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Migrate Data", |ui|{
            if ui.button("Move One Table").clicked() {

            }
        });
    }
}
