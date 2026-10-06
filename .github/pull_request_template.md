<!--
Thanks for the pull request.

Please make sure:
  - the change is focused, and you have explained *why* below
  - `cargo fmt --all -- --check` and
    `cargo clippy --workspace --all-targets --all-features -- -D warnings` pass
  - `cargo test --workspace` passes
  - if the change is user-visible, a `.changes/<name>.md` file is added listing
    BOTH the crate and the npm package with the SAME bump level
-->

## What does this change?

<!-- A short description. Link any related issue with "Closes #123". -->

## Which plugin(s) does it affect?

- [ ] `tauri-plugin-beep`
- [ ] `tauri-plugin-hash`
- [ ] `tauri-plugin-hostname`
- [ ] `tauri-plugin-mimetype`
- [ ] `tauri-plugin-timezone`
- [ ] `tauri-plugin-uuid`
- [ ] repository / tooling / docs only

## Type of change

- [ ] Bug fix (patch)
- [ ] New feature, backwards compatible (minor)
- [ ] Breaking change (major)
- [ ] Documentation / tooling only

## How has this been tested?

<!-- Commands you ran, and on which platform(s). -->

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `npx prettier --check .` passes
- [ ] A `.changes/<name>.md` entry lists both the crate and the npm package
- [ ] If the public API changed, the plugin `README.md` and `CHANGELOG.md` were updated
