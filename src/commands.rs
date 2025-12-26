use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::InAppReviewExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.in_app_review().ping(payload)
}

#[command]
pub(crate) async fn request_review<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.in_app_review().request_review()
}
