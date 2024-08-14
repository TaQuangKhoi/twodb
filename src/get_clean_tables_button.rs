use std::env::var;
use std::thread;
use eframe::emath::Align2;
use egui::Ui;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use crate::TwoDBApp;
use crate::working_database::get_clean_tables;

impl TwoDBApp {
    pub fn render_clean_tables_button(&mut self, ui: &mut Ui) {
        let button = ui.button("Get Clean Tables");
        if button.clicked() {
            self.is_busy = true;
            ui.close_menu();
            self.button_get_clean_tables_event();
        }
    }

    fn button_get_clean_tables_event(&mut self) {
        thread::spawn(move || {
            let mut thread_toasts = Toasts::new()
                .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
                .direction(egui::Direction::BottomUp);

            let database_name = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            get_clean_tables(&database_name);

            let database_name = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            get_clean_tables(&database_name);

            let text = format!("Done Get Clean Tables for {}", database_name);
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