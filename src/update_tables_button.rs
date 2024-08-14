use crate::TwoDBApp;

impl TwoDBApp {
    pub fn render_update_tables_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Update Tables").clicked() {
            self.is_busy = true;
            ui.close_menu();
            self.button_update_tables_event();
        }
    }

    fn button_update_tables_event(&mut self) {
        todo!()
    }
}