# Security Policy

## Supported Versions

Currently supported versions of Notepad++ Rust Edition:

| Version | Supported          |
| ------- | ------------------ |
| 8.x.x   | :white_check_mark: |
| < 8.0   | :x:                |

We recommend always using the latest release for the best security and features.

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please report it responsibly:

### 1. Contact Information

Send details to: **security@notepadplusplus-rust.org**
(Alternative: Create a private security advisory on GitHub)

### 2. What to Include

Please include the following in your report:

- **Description**: Clear description of the vulnerability
- **Impact**: What could an attacker accomplish?
- **Affected Versions**: Which versions are affected?
- **Reproduction Steps**: Detailed steps to reproduce
- **Proof of Concept**: Code or examples (if applicable)
- **Suggested Fix**: If you have ideas (optional)

### 3. Response Timeline

- **24-48 hours**: Initial acknowledgment
- **7 days**: Initial assessment and severity classification
- **30 days**: Fix development and testing (for critical issues)
- **90 days**: Public disclosure (coordinated with reporter)

### 4. Severity Levels

We classify vulnerabilities using CVSS v3.1:

- **Critical** (9.0-10.0): Immediate fix, emergency release
- **High** (7.0-8.9): Fix within 30 days, patch release
- **Medium** (4.0-6.9): Fix within 90 days, next release
- **Low** (0.1-3.9): Fix when convenient, next major release

### 5. Our Commitment

We commit to:

- Acknowledge your report promptly
- Keep you informed about fix progress
- Credit you in the security advisory (if desired)
- Coordinate public disclosure timing with you

## Security Best Practices

For users of Notepad++ Rust Edition:

### Recommended Settings

1. **Keep Updated**: Always use the latest version
   ```bash
   # Check for updates regularly
   notepad-plus --version
   ```

2. **Verify Downloads**:
   - Download only from official sources
   - Verify checksums (SHA-256)
   - Check GPG signatures (when available)

3. **Safe File Handling**:
   - Be cautious opening files from untrusted sources
   - Disable auto-execution of macros/scripts (future feature)
   - Scan files for malware before opening

### For Developers

1. **Dependency Auditing**:
   ```bash
   # Run cargo-audit regularly
   cargo install cargo-audit
   cargo audit
   ```

2. **No Unsafe Code** (unless necessary):
   - All unsafe code must have `// SAFETY:` comments
   - Platform crates may use unsafe for FFI only
   - Portable crates must be 100% safe

3. **Input Validation**:
   - Validate all file paths
   - Sanitize user input
   - Bounds check all operations

4. **Error Handling**:
   - Use `Result<T>` for all fallible operations
   - Never use `.unwrap()` in production paths
   - Log errors securely (no sensitive data)

## Known Security Considerations

### Current Scope

Notepad++ Rust Edition is a **text editor**, not a:
- Web browser (no JavaScript execution)
- Script interpreter (no macro execution yet)
- Network application (no remote connections)

### Future Considerations

When we add plugins:
- Plugins will be sandboxed
- Permissions model will be implemented
- Code signing may be required

## Security Hall of Fame

We thank the following researchers for responsibly disclosing vulnerabilities:

<!-- Names will be added here as vulnerabilities are found and fixed -->
- *No vulnerabilities reported yet*

## Additional Resources

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CWE Top 25](https://cwe.mitre.org/top25/)

## Questions?

For general security questions (not vulnerabilities), open a discussion on GitHub:
https://github.com/GunwantSaini/TestNotepadPlusPlus/discussions

---

**Last Updated**: 2025-01-01
**Version**: 1.0
