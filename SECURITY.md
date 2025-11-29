# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of MXP Python bindings seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

📧 **security@getmxp.xyz**

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

### What to Include

Please include the following information in your report:

- **Type of issue** (e.g., buffer overflow, injection, crash, etc.)
- **Full paths of source file(s)** related to the issue
- **Location of the affected source code** (tag/branch/commit or direct URL)
- **Step-by-step instructions** to reproduce the issue
- **Proof-of-concept or exploit code** (if possible)
- **Impact of the issue** and how an attacker might exploit it
- **Python and Rust versions** used

### What to Expect

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
2. **Initial Assessment**: Within 7 days, we will provide an initial assessment
3. **Regular Updates**: We will keep you informed of our progress
4. **Resolution**: We will notify you when the issue is fixed
5. **Credit**: With your permission, we will credit you in the security advisory

### Disclosure Policy

- We follow a **90-day disclosure timeline** from initial report to public disclosure
- We will coordinate with you on the disclosure timeline
- We may request an extension if a fix requires more time
- We will publish a security advisory on GitHub once the fix is released

## Security Best Practices

### For Users

1. **Keep Updated**: Always use the latest version
2. **Verify Checksums**: Verify package integrity from PyPI
3. **Dependency Scanning**: Use tools like `safety` to scan dependencies
4. **Secure Environment**: Use virtual environments

### For Developers

1. **Input Validation**: Validate all data passed to MXP
2. **Error Handling**: Handle all errors gracefully
3. **Memory Safety**: Trust Rust's safety but test edge cases
4. **Fuzzing**: Fuzz test your usage of the library

## Contact

- **Security Issues**: security@getmxp.xyz
- **General Questions**: support@yafa.dev

---

Thank you for helping keep MXP Python bindings and its users safe!
