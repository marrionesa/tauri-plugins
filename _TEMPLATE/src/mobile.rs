// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::Error;

/// Holds runtime state for the mobile variant of the plugin. Stored in
/// `tauri::Manager::state` after `init()` runs. The `handle` field is the
/// bridge to the native Kotlin/Swift plugin instance.
pub struct Template<R: Runtime> {
    #[allow(dead_code)]
    app: AppHandle<R>,
    #[allow(dead_code)]
    handle: tauri::plugin::PluginHandle<R>,
}

impl<R: Runtime> Template<R> {
    pub(crate) fn init(
        app: &AppHandle<R>,
        api: PluginApi<R, Error>,
    ) -> crate::Result<Self> {
        #[cfg(target_os = "android")]
        let handle = api.register_android_plugin("com.plugin.template", "ExamplePlugin")?;
        #[cfg(target_os = "ios")]
        let handle = api.register_ios_plugin(init_plugin_template)?;

        Ok(Self {
            app: app.clone(),
            handle,
        })
    }
}

/// iOS binding symbol — must match `init_plugin_<name>` so the Swift runtime
/// can find it.
#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_template);
