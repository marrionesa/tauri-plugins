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

`tauri-plugin-hostname` exposes two commands that take no user input and return
read-only identifiers sourced from the OS via the `gethostname` and `whoami`
crates. The security-relevant properties are:

- **No input parsing**: both commands accept no arguments, so there is no attack
  surface for injection or fuzzing.
- **No I/O**: the commands read from process-local libc / Win32 APIs only
  (`libc::gethostname` / `GetComputerNameW` via `gethostname`, and
  `getpwuid_r` / `GetUserNameW` via `whoami`). They do not touch the
  filesystem, network, or process table.
- **No side effects**: the commands do not mutate any state observable outside
  the calling thread.
- **Returned values are low-sensitivity machine/user identifiers**: the hostname
  uniquely identifies the machine and the username identifies the user. Treat
  them as low-sensitivity but not zero-sensitivity. Do NOT log them to a remote
  telemetry endpoint without explicit user consent — they can fingerprint the
  user when combined with other signals.
- **Restrictable via capability**: if your app's JS sandbox should not see
  either value, declare `hostname:deny-hostname` or `hostname:deny-username`
  in the relevant capability file. See [`permissions/`](./permissions/) for
  the full list.

If you discover any property of this plugin that contradicts the above,
**please report it privately** before opening a public issue.
