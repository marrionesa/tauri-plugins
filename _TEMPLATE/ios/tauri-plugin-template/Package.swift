// swift-tools-version:5.5
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//
// Swift Package Manager manifest for the iOS / macOS side of the plugin.
// Mirrors the upstream scaffolded `ios-spm/Package.swift` from the Tauri CLI
// plugin template (see crates/tauri-cli/templates/plugin/ios-spm/Package.swift).
//
// The Tauri runtime expects the package name to match `tauri-plugin-<name>`
// exactly, and the target to be a `.static` library named identically.

import PackageDescription

let package = Package(
    name: "tauri-plugin-template",
    platforms: [
        .macOS(.v10_13),
        .iOS(.v15),
    ],
    products: [
        .library(
            name: "tauri-plugin-template",
            type: .static,
            targets: ["tauri-plugin-template"]
        ),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri_api")
    ],
    targets: [
        .target(
            name: "tauri-plugin-template",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources"
        )
    ]
)
