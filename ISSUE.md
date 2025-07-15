# Issue Reporting Guide

Thank you for helping improve Winix! Please follow these steps to create a clear, actionable issue.

## Steps to Create an Issue

1. **Go to the Issues tab** on GitHub.
2. **Click "New Issue"**.
3. **Choose the appropriate issue template** (Bug, Feature Request, Documentation, etc.).
4. **Fill out the issue form** using the format below.

---

## Issue Format

### Title
- Be concise and descriptive (e.g., "[BUG] Crash when running chmod on symlink").

### Description
- What is the problem or request?
- Steps to reproduce (for bugs)
- Expected behavior
- Actual behavior
- Screenshots or logs (if applicable)

### Environment
- OS version
- Winix version
- Rust version (if building from source)

### Additional Context
- Any other relevant information

---

## Tagging Your Issue

Please add relevant labels to help us triage your issue:
- `bug`: For errors, crashes, or unexpected behavior
- `feature`: For new feature requests
- `documentation`: For docs improvements
- `help wanted`: If you need guidance or want to collaborate
- `good first issue`: For beginner-friendly tasks
- `question`: For general questions

If unsure, leave tagging to maintainers.

---

## Example Issue

**Title:** [BUG] Incorrect output for ps command on Windows 11

**Description:**
When running `ps`, the output does not show all running processes. Steps to reproduce:
1. Open terminal
2. Run `winix ps`
3. Observe missing processes

**Environment:**
- OS: Windows 11
- Winix: v1.2.0
- Rust: 1.70.0

**Additional Context:**
No error logs, but output is incomplete.

---

Thank you for your feedback and for helping make Winix better!
