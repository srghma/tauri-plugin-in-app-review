use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<InAppReview<R>> {
    Ok(InAppReview(app.clone()))
}

/// Access to the in-app-review APIs.
pub struct InAppReview<R: Runtime>(AppHandle<R>);

impl<R: Runtime> InAppReview<R> {
    pub fn request_review(&self) -> crate::Result<()> {
        Err(crate::Error::from(std::io::Error::other(
            "In App Review Not Supported This Platform.",
        )))
    }
}
