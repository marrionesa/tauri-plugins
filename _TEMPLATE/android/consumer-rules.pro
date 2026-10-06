# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT
#
# ProGuard consumer rules applied to apps that consume this library. Keep any
# reflection-based entry points (Kotlin classes referenced by name from Rust
# via `register_android_plugin`) from being obfuscated away.

-keep class com.plugin.template.** { *; }
