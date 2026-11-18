use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<PrehniteData<R>> {
  Ok(PrehniteData(app.clone()))
}

/// Access to the prehnite-data APIs.
pub struct PrehniteData<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PrehniteData<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
