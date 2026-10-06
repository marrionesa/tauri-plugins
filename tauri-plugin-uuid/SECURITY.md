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

`tauri-plugin-uuid` exposes two commands that take no user input and produce
random UUIDs sourced from the OS CSPRNG (via the `uuid` crate's `getrandom`
backend). The security-relevant properties are:

- **No input parsing**: the commands accept no arguments, so there is no attack
  surface for injection or fuzzing.
- **CSPRNG source**: `Uuid::new_v4()` and `Uuid::now_v7()` both call
  `getrandom::getrandom()` on Linux/Android, `BCryptGenRandom` on Windows, and
  `SecRandomCopyBytes` on macOS/iOS. None of these are user-influenceable.
- **No I/O**: the commands do not touch the filesystem, network, or process
  table.
- **No side effects**: the commands do not mutate any state observable outside
  the calling thread.

If you discover any property of this plugin that contradicts the above,
**please report it privately** before opening a public issue.
