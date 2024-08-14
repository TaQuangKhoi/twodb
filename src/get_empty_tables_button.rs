use std::env::var;
use std::thread;
use eframe::emath::Align2;
use egui::Ui;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use crate::TwoDBApp;
use crate::working_database::get_empty_tables;

impl TwoDBApp {
    pub fn render_get_empty_tables_button(&mut self, ui: &mut Ui) {
        let button = ui.button("Get Empty Tables for Source");
        if button.clicked() {
            self.is_busy = true;
            ui.close_menu();
            self.get_empty_tables_event();
        }
    }

    fn get_empty_tables_event(&mut self) {
        thread::spawn(move || {
            let mut thread_toasts = Toasts::new()
                .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
                .direction(egui::Direction::BottomUp);
            let database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            get_empty_tables(&database_name);
            let text = format!("Done Get **Empty** Tables for {}", database_name);
            thread_toasts.add(Toast {
                text: text.into(),
                kind: ToastKind::Success,
                options: ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true),
                ..Default::default()
            });
        });
    }
}