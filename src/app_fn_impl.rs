use std::sync::{Arc, Mutex};
use log::info;
use crate::TwoDBApp;

impl TwoDBApp {
    pub fn notify(text: String, is_busy: Arc<Mutex<bool>>, toast_text: Arc<Mutex<String>>) {
        info!("{}", text);
        *is_busy.lock().unwrap() = false;
        *toast_text.lock().unwrap() = text;
    }
}