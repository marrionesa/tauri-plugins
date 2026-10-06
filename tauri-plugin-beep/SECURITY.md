# Security Policy

**Do not report security vulnerabilities through public GitHub issues.**

**Please use the
[Private Vulnerability Disclosure](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability#privately-reporting-a-security-vulnerability)
feature of GitHub.**

Include as much of the following information:

- Type of issue (e.g. improper input parsing, privilege escalation, etc.)
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- The distribution affected or used to help us with reproduction of the issue
- Step-by-step instructions to reproduce the issue
- Ideally a reproduction repository
- Impact of the issue, including how an attacker might exploit the issue

## Contact

Please disclose a vulnerability or security-relevant issue here:
<https://github.com/marrionesa/tauri-plugins/security/advisories/new>

Alternatively, contact the maintainer of this plugin via the repository's
security advisory form linked above.

## Security considerations for this plugin

`tauri-plugin-beep` exposes a single command, `beep()`, which plays the
operating system's native alert sound. The security-relevant properties are:

- **No input parsing**: `beep()` takes no arguments — there is no attack
  surface for injection, fuzzing, or path traversal.
- **No shell interpolation**: on Linux and macOS the plugin spawns a
  subprocess (`paplay` / `afplay`), but the executable name and its single
  argument are **hardcoded string literals** in `src/commands.rs`. There is
  no `sh -c`, no `Shell::Spawn`, and no user-controlled content reaches the
  argv vector.
- **Windows FFI only**: on Windows the plugin calls `MessageBeep` directly via
  the `windows-sys` FFI — no subprocess is spawned.
- **No filesystem writes**: the plugin does not create, modify, or delete any
  files. It reads no files either (the audio is supplied by the OS).
- **No network**: the plugin performs no network I/O of any kind.
- **No user-controlled content played**: the beep sound itself is supplied by
  the OS — there is no audio path through which an attacker could exfiltrate
  data or trigger arbitrary decoding.
- **Subprocess lifetime**: on Linux/macOS the spawned `paplay` / `afplay`
  process is fire-and-forget (we `spawn()` it and do not `wait()`). It inherits
  null stdio handles, so it cannot read the parent's stdin or pollute the
  parent's stdout/stderr. The child becomes orphaned and is reaped by init.

If you discover any property of this plugin that contradicts the above,
**please report it privately** before opening a public issue.
