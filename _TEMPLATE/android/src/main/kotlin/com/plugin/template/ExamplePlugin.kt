// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//
// Kotlin entry point of the Android side of the plugin. The class name
// "ExamplePlugin" is referenced from `src/mobile.rs` via:
//
//     api.register_android_plugin("com.plugin.template", "ExamplePlugin")
//
// When you rename the plugin, also rename the package folder
// `android/src/main/kotlin/com/plugin/template/` to match the new namespace,
// and update the `register_android_plugin("com.plugin.<new-name>", "...")`
// call in `src/mobile.rs`.

package com.plugin.template

import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin

@TauriPlugin
class ExamplePlugin(private val plugin: Plugin) {

    @Command
    fun ping(value: String): HashMap<String, Any> {
        val result = HashMap<String, Any>()
        result["value"] = value
        return result
    }
}
