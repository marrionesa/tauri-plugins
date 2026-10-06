# tauri-plugin-template

REPLACE ME — short one-line description of what this plugin does.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Install

_This plugin requires a Rust version of at least **1.90**_

There are three general methods of installation that we can recommend:

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from GitHub using git tags / revision hashes (most secure)
3. Git submodule install this repo in your Tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Rust crate by adding the following to your `Cargo.toml`:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-template = "0.1.0"
# alternatively with Git:
tauri-plugin-template = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-template
# or
npm add @marrionesa/plugin-template
# or
yarn add @marrionesa/plugin-template
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_template::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { ping } from '@marrionesa/plugin-template'

const reply = await ping('hello')
console.log(reply) // { value: 'hello' }
```

## Permissions

The plugin ships a default permission set that enables the `ping` command. To
restrict or extend access, declare a capability in
`src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["template:allow-ping"]
}
```

See [`permissions/`](./permissions/) for the full list of available
permissions.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.
