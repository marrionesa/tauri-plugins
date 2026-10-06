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

`tauri-plugin-mimetype` exposes two commands that take a single string
argument (a file extension or a file path) and return an `Option<String>` by
looking the input up in the pure-Rust `mime_guess` crate's static table. The
security-relevant properties are:

- **No input parsing of untrusted content** beyond `mime_guess`'s own lookup
  table: the input string is only used to index into a static `&str → &str`
  map. There is no regex, no format-string handling, no deserialization of
  user-controlled bytes.
- **No filesystem access**: `from_path` only inspects the _string_ it receives
  via `Path::extension()`. It does not open, stat, read, or otherwise touch
  the file at that path. The path does not need to exist on disk.
- **No network access**.
- **No side effects**: the commands do not mutate any state observable outside
  the calling thread. They allocate only the returned `Option<String>`.

If you discover any property of this plugin that contradicts the above,
**please report it privately** before opening a public issue.
