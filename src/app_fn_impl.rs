use std::sync::{Arc, Mutex};
use crate::TwoDBApp;

impl TwoDBApp {
    pub fn notify(text: String, is_busy: Arc<Mutex<bool>>, toast_text: Arc<Mutex<String>>) {
        println!("{}", text);
        *is_busy.lock().unwrap() = false;
        *toast_text.lock().unwrap() = text;
    }
}