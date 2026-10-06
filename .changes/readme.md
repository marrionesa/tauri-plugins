# Changes

This folder follows the upstream Tauri convention: every PR that warrants a
version bump adds a markdown file here, picked up by
[covector](https://github.com/jbolda/covector) at release time.

## File format

Create a file with any name ending in `.md` (e.g. `add-uuid-v7.md`) and use the
following frontmatter:

```md
---
'tauri-plugin-<name>': patch # or minor | major | minor:feat | major:breaking
'tauri-plugin-<name>-js': patch # SAME increment as the Rust package
---

A short human-readable summary of what changed and why.
```

## Critical rule (verbatim from upstream `tauri-apps/plugins-workspace`)

> In this repository, even if only the Rust code or only the JavaScript code of
> a plugin changed, **both packages need to be bumped with the same increment**.

So every change file MUST list both the Rust crate (`tauri-plugin-<name>`) and
the JavaScript package (`tauri-plugin-<name>-js`).

## Bump levels

| Keyword          | Meaning                                                              |
| ---------------- | -------------------------------------------------------------------- |
| `patch`          | Bug fix, docs, internal refactor — no API change.                    |
| `minor`          | New command/feature added, no breaking change.                       |
| `minor:feat`     | Same as `minor` but explicitly tagged as a feature in the changelog. |
| `major`          | Breaking change (e.g. removed command, renamed API).                 |
| `major:breaking` | Same as `major`, explicit breaking tag.                              |

## Example

```md
---
'tauri-plugin-uuid': minor:feat
'tauri-plugin-uuid-js': minor:feat
---

Add generate_v7 command for time-ordered UUIDs. Backwards compatible.
```

## Publishing flow

1. A maintainer runs `covector` locally.
2. For each plugin:
   - Rust: `cargo publish --no-verify` → <https://crates.io>
   - JavaScript: `pnpm build && npm publish --provenance --access public` → <https://npmjs.com>
3. Git tags are pushed automatically:
   - `<plugin-name>-v<x.y.z>` for the Rust crate.
   - `<plugin-name>-js-v<x.y.z>` for the JavaScript package.
