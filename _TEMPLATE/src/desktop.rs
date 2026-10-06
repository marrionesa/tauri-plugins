// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Desktop-only state and helpers. Compiled only on `#[cfg(desktop)]`.

use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::Error;

/// Holds runtime state for the desktop variant of the plugin. Stored in
/// `tauri::Manager::state` after `init()` runs.
pub struct Template<R: Runtime> {
    #[allow(dead_code)]
    app: AppHandle<R>,
}

impl<R: Runtime> Template<R> {
    pub(crate) fn init(app: &AppHandle<R>, _api: PluginApi<R, crate::Error>) -> crate::Result<Self> {
        let _app = app.clone();
        // Add any platform-specific initialization here (e.g. spawn a thread,
        // open a handle, register OS observers).
        Ok(Self { app: _app })
    }

    /// Trivial accessor — useful if commands need to call desktop-only helpers.
    #[allow(dead_code)]
    pub fn app(&self) -> &AppHandle<R> {
        &self.app
    }
}

impl<R: Runtime> Drop for Template<R> {
    fn drop(&mut self) {
        // Cleanup any platform resources here.
        let _ = self.app;
    }
}

#[allow(dead_code)]
fn into_anyhow<E: std::fmt::Display>(e: E) -> Error {
    Error::Anyhow(e.to_string())
}
