use std::sync::{Arc, Mutex};

pub struct AppState {}

impl Default for AppState {
    fn default() -> Self {
        AppState {}
    }
}

/// A wrapper struct around a mutex holding the global app state. This version of Tauri only accepts a tuple
/// struct (one with unnamed members) as data it can manage.
pub struct App(pub Arc<Mutex<AppState>>);

impl Default for App {
    fn default() -> Self {
        App(Arc::new(Mutex::new(AppState::default())))
    }
}
