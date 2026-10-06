// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//
// Swift entry point of the iOS side of the plugin. The function
// `init_plugin_template` is referenced from `src/mobile.rs` via:
//
//     tauri::ios_plugin_binding!(init_plugin_template)
//
// When you rename the plugin, also rename the Swift target folder
// `ios/tauri-plugin-template/` and update the binding symbol in
// `src/mobile.rs` to `init_plugin_<name>`.

import Tauri

@_cdecl("init_plugin_template")
public func init_plugin_template(webview: WKWebView) {
    let plugin = ExamplePlugin()
    Tauri.registerPlugin(webview: webview, name: "template", plugin: plugin)
}

public class ExamplePlugin: Plugin {
    @objc public func ping(_ value: String) -> [String: Any] {
        return ["value": value]
    }
}
