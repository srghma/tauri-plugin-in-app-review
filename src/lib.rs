use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::InAppReview;
#[cfg(mobile)]
use mobile::InAppReview;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the in-app-review APIs.
pub trait InAppReviewExt<R: Runtime> {
    fn in_app_review(&self) -> &InAppReview<R>;
}

impl<R: Runtime, T: Manager<R>> crate::InAppReviewExt<R> for T {
    fn in_app_review(&self) -> &InAppReview<R> {
        self.state::<InAppReview<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("in-app-review")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
            commands::request_review
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let in_app_review = mobile::init(app, api)?;
            #[cfg(desktop)]
            let in_app_review = desktop::init(app, api)?;
            app.manage(in_app_review);
            Ok(())
        })
        .build()
}
