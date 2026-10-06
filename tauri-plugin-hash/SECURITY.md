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

`tauri-plugin-hash` exposes two commands (`hash_string`, `hash_file`) that
accept arbitrary user-supplied input. The security-relevant properties are:

- **Constant-memory streaming**: `hash_file` reads the file in 8 KB chunks via
  `tokio::fs::File::read`, so memory usage is constant regardless of file size.
  No matter how large the input, the plugin cannot OOM the process.
- **Timing-safe comparison internals**: the underlying `sha2` and `md-5` crates
  use constant-time comparisons internally where relevant, and the digest
  output is hex-encoded — there is no secret-dependent comparison that an
  attacker could time.
- **No algorithm validation by allowlist**: any algorithm string other than
  `"sha256"`, `"sha512"`, `"md5"` (case-insensitive) returns an `Anyhow` error
  before any hashing begins.
- **`hash_file` reads arbitrary paths**: the caller supplies the `path`
  argument and the plugin will `open(2)` it with the Tauri process's
  permissions. If you do not want JavaScript to be able to hash arbitrary files
  (e.g. `/etc/shadow`, application data, etc.), **deny `allow-hash-file`** in
  a capability. `hash_string` is always safe (takes a string, no I/O).
- **No network access, no subprocess spawning, no side effects** beyond reading
  the file the caller specifies.

If you discover any property of this plugin that contradicts the above,
**please report it privately** before opening a public issue.
