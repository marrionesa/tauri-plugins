# Contributing

First off, thank you for considering contributing to this Tauri plugin workspace.
This file mirrors the contributor expectations of the upstream Tauri repos so that
any plugin produced here is **submission-ready** for `tauri-apps/plugins-workspace`.

## TL;DR

| Requirement                                                                      | Status in this workspace                        |
| -------------------------------------------------------------------------------- | ----------------------------------------------- |
| Sign your commits (GPG/SSH/X.509)                                                | mandatory                                       |
| No Contributor License Agreement (CLA)                                           | inbound = outbound                              |
| Dual `Apache-2.0 OR MIT` license                                                 | enforced in every `Cargo.toml` + `package.json` |
| SPDX headers on every Rust file                                                  | enforced                                        |
| Conventional-ish PR title (`fix:`/`feat:`/`chore:`)                              | recommended, not enforced                       |
| Covector change file under `.changes/` for every release PR                      | mandatory                                       |
| Rust version `1.90`, edition `2021`                                              | enforced in `Cargo.toml`                        |
| `cargo fmt --check`, `cargo clippy -D warnings`, `taplo fmt`, `prettier --check` | enforced in CI                                  |

## Before you start

1. **Open a suggestion issue first** if you intend to ship a brand-new plugin.
   The Tauri maintainers greenlight scope before code is written.
2. **Read the upstream guide**:
   <https://raw.githubusercontent.com/tauri-apps/plugins-workspace/v2/.github/CONTRIBUTING.md>
3. **Read the Code of Conduct**: [`CODE_OF_CONDUCT.md`](./CODE_OF_CONDUCT.md).
4. **Sign your commits**: see
   <https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits>.

## Commit messages

The upstream Tauri repos do NOT enforce strict Angular-style conventional commits,
but they use squash-merges so the PR title becomes the commit message. We follow
the same convention:

- `feat(<plugin>): add X` — new feature in a plugin
- `fix(<plugin>): correct Y (fix #1234)` — bug fix, issue reference in parentheses
- `chore(deps): bump uuid to 1.16` — dependency bumps
- `docs(<plugin>): improve README` — doc-only changes
- `ci: cache cargo registry` — CI changes

## Releases — covector

Every PR that warrants a version bump MUST add a file under `.changes/`:

```md
---
'tauri-plugin-uuid': minor:feat
'tauri-plugin-uuid-js': minor:feat
---

Add generate_v7 command for time-ordered UUIDs.
```

**Critical rule** (verbatim from upstream):

> In this repository, even if only the Rust code or only the JavaScript code of
> a plugin changed, both packages need to be bumped with the same increment!

So every change file MUST list both `<name>` (Rust) and `<name>-js` (JavaScript)
entries with the same `patch` / `minor` / `major` increment.

## Pull request checklist

- [ ] Commits are signed (`git log --show-signature`).
- [ ] `cargo fmt --all -- --check` passes.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.
- [ ] `taplo fmt --check --diff` passes (TOML formatting).
- [ ] `pnpm format:check` passes (prettier on `guest-js`).
- [ ] If the change is user-visible, a `.changes/<name>.md` file is added listing
      both the Rust and JS package with the same bump level.
- [ ] `[package.metadata.platforms.support]` in `Cargo.toml` reflects reality.
- [ ] README platform support table matches the metadata above.

## Security disclosures

**Do not report security vulnerabilities through public GitHub issues.**
See [`SECURITY.md`](./SECURITY.md) for the private disclosure path.
