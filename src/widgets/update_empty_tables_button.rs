use std::env::var;
use std::thread;
use eframe::emath::Align2;
use egui::Ui;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use crate::TwoDBApp;
use crate::working_database::update_empty_tables;

impl TwoDBApp {
    pub fn render_get_empty_tables_button(&mut self, ui: &mut Ui) {
        if ui.button("Update Empty Tables").clicked() {
            self.is_busy_old = true;
            ui.close_menu();
            self.get_empty_tables_event();
        }
    }

    fn get_empty_tables_event(&mut self) {
        let is_busy = self.is_busy.clone();
        *is_busy.lock().unwrap() = true;
        thread::spawn(move || {
            // let mut thread_toasts = Toasts::new()
            //     .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
            //     .direction(egui::Direction::BottomUp);

            let database_name_source = var("POSTGRES_DB_SOURCE").unwrap_or(String::from(""));
            update_empty_tables(&database_name_source);

            let database_name_target = var("POSTGRES_DB_TARGET").unwrap_or(String::from(""));
            update_empty_tables(&database_name_target);

            let text = format!("Done Get **Empty** Tables for {} and {}", database_name_source, database_name_target);
            // thread_toasts.add(Toast {
            //     text: text.clone().into(),
            //     kind: ToastKind::Success,
            //     options: ToastOptions::default()
            //         .duration_in_seconds(5.0)
            //         .show_progress(true),
            //     ..Default::default()
            // });
            println!("{}", text);
            *is_busy.lock().unwrap() = false;
        });
    }
}