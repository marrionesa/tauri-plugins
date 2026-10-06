// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! REPLACE ME — short module-level description of what this plugin does.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_template::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { ping } from '@marrionesa/plugin-template'
//! const reply = await ping('hello')
//! ```

#![doc(
    html_logo_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    html_favicon_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png"
)]

use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

mod commands;
mod error;
#[cfg(mobile)]
mod mobile;
#[cfg(desktop)]
mod desktop;

pub mod models;
pub use error::{Error, Result};
pub use models::*;

/// Initializes the plugin.
///
/// Register this on your `tauri::Builder::default().plugin(...)` chain.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("template")
        .invoke_handler(tauri::generate_handler![
            commands::ping,
        ])
        .setup(|app, _api| {
            #[cfg(mobile)]
            {
                let template = mobile::init(app, _api)?;
                app.manage(template);
            }
            #[cfg(desktop)]
            {
                let template = desktop::init(app, _api)?;
                app.manage(template);
            }
            Ok(())
        })
        .build()
}
