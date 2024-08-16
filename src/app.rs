use egui::Align2;
use egui_toast::{Toasts};
use std::sync::{Arc, Mutex};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TwoDBApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    pub(crate) window_open: bool,
    pub is_busy_old: bool, // This field is for Spinner

    pub is_busy: Arc<Mutex<bool>>, // for synchronize thread

    pub toast_text: Arc<Mutex<String>>,
    selected : Enum,
}

impl Default for TwoDBApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World! 2".to_owned(),
            value: 2.6,
            window_open: false,
            is_busy_old: false,
            is_busy: Arc::new(Mutex::new(false)),
            toast_text: Arc::new(Mutex::new("".to_owned())),
            selected: Enum::First,
        }
    }
}

impl TwoDBApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TwoDBApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            {
                /// Reset is_busy to false
                let is_busy = app.is_busy.clone();
                *is_busy.lock().unwrap() = false;

                app.window_open = false;

                app.toast_text.lock().unwrap().clear();
            }

            return app;
        }

        Default::default()
    }
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
enum Enum {
    First,
    Second,
    Third,
}

impl eframe::App for TwoDBApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut toasts = Toasts::new()
            .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0)) // 10 units from the bottom right corner
            .direction(egui::Direction::BottomUp);

        if self.window_open {
            egui::Window::new("Choose a table")
                .open(&mut self.window_open)
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("contents");
                    egui::ComboBox::from_label("Select one!")
                        .selected_text(format!("{:?}", self.selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected, Enum::First, "First 1");
                            ui.selectable_value(&mut self.selected, Enum::Second, "Second 2");
                            ui.selectable_value(&mut self.selected, Enum::Third, "Third 3");
                        },
                        );
                });
        }

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("Update", |ui| {
                        self.render_update_tables_button(ui);
                        self.render_update_self_referencing_tables_button(ui);
                        self.render_clean_tables_button(ui);
                        self.render_get_empty_tables_button(ui);
                    });
                    self.menu_btn_migrate_data_render(ctx, ui);
                    ui.menu_button("Settings", |ui| {});

                    if self.is_busy.lock().unwrap().clone() {
                        ui.add(egui::Spinner::new());
                    }

                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Clean Tables");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        let mut toast_text = self.toast_text.lock().unwrap().clone();
        if !toast_text.is_empty() {
            let text = toast_text.clone();
            toasts.add(egui_toast::Toast {
                text: text.into(),
                kind: egui_toast::ToastKind::Success,
                options: egui_toast::ToastOptions::default()
                    .duration_in_seconds(5.0)
                    .show_progress(true),
                ..Default::default()
            });
            self.toast_text.lock().unwrap().clear();
        }

        toasts.show(ctx);
    }

    /// Called by the framework to save state before shutdown.
    /// Notes by Keios: this fn can be call despite the app is not closing
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
