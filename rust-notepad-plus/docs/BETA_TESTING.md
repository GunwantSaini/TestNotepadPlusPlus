# Beta Testing Guide - Notepad++ Rust Edition

Version 8.0.0 Beta

## Welcome Beta Testers!

Thank you for participating in the Notepad++ Rust Edition beta program. Your feedback is invaluable in helping us create a stable, high-quality release.

## Table of Contents

1. [Beta Program Overview](#beta-program-overview)
2. [Getting Started](#getting-started)
3. [Testing Priorities](#testing-priorities)
4. [Test Scenarios](#test-scenarios)
5. [Reporting Issues](#reporting-issues)
6. [Known Issues](#known-issues)
7. [Feedback Channels](#feedback-channels)

---

## Beta Program Overview

### What We're Testing

**Phase 4 Beta (Current):**
- Core file operations (open, save, close)
- Multi-document management
- Search and replace functionality
- Basic editing features
- Windows installer and deployment
- Performance and stability

**What's NOT in This Beta:**
- Plugins system (planned for Phase 6)
- Multi-tab UI (planned for Phase 5)
- Syntax highlighting (partial, Phase 5)
- Macros (planned for Phase 5)
- Linux/macOS builds (Phase 7)

### Timeline

- **Beta Start:** 2024-03-17
- **Beta Duration:** 3 weeks
- **Feature Freeze:** Week 2
- **Release Candidate:** Week 3
- **Stable Release:** TBD

### Expectations

**As a Beta Tester, we ask you to:**
- ✅ Test daily tasks and workflows
- ✅ Report bugs with detailed information
- ✅ Try to reproduce issues
- ✅ Provide constructive feedback
- ✅ Test on your actual work files (after backup!)
- ✅ Be patient with rough edges

**What You Can Expect:**
- 🐛 Some bugs and crashes
- ⚠️ Missing features compared to original Notepad++
- 📊 Performance may vary
- 🔄 Frequent updates during beta period
- 💬 Active communication from dev team

---

## Getting Started

### Installation

**Windows Beta:**
1. Download `notepad-plus-rust-setup-beta.exe`
2. **IMPORTANT:** Install to a separate location from stable Notepad++
3. Recommended: `C:\Program Files\Notepad++ Rust Beta\`
4. Do NOT set as default text editor yet

**Backup First!**
Before testing with real files:
```bash
# Backup your documents
xcopy /E /I /Y C:\MyDocuments\ C:\MyDocuments_Backup\

# Or use built-in backup
# Settings → Backup → Enable Backup → Set backup directory
```

### First Launch Checklist

- [ ] Application launches without errors
- [ ] Window appears correctly positioned
- [ ] Menu bar is accessible
- [ ] Toolbar icons are visible
- [ ] Status bar shows correctly
- [ ] Can type in editor
- [ ] Can create new file (`Ctrl+N`)

### Basic Smoke Test (5 minutes)

1. **Create File:**
   - `Ctrl+N` → Type "Hello World" → `Ctrl+S`
   - Save as `test.txt`
   - ✓ File saved successfully

2. **Open File:**
   - `Ctrl+O` → Open `test.txt`
   - ✓ Content appears correctly

3. **Edit File:**
   - Add text → `Ctrl+Z` (undo) → `Ctrl+Y` (redo)
   - ✓ Undo/redo works

4. **Search:**
   - `Ctrl+F` → Search for "World"
   - ✓ Text is found and highlighted

5. **Close:**
   - Close file without saving
   - ✓ Prompt to save appears
   - ✓ Application closes cleanly

**If all steps pass:** Proceed with detailed testing
**If any step fails:** Report immediately as P0 (Critical) bug

---

## Testing Priorities

### Priority 0: Critical (Must Fix Before Release)

These issues prevent basic usage:
- Application crashes on launch
- Cannot open any files
- Cannot save files (data loss risk)
- Total UI failure
- Security vulnerabilities

### Priority 1: High (Should Fix Before Release)

Significantly impact user experience:
- Frequent crashes during normal use
- Data corruption
- Major performance issues
- Core features not working (search, replace, undo)
- Memory leaks

### Priority 2: Medium (Nice to Fix)

Noticeable but workaroundable:
- Minor UI glitches
- Rare crashes
- Performance hiccups
- Edge case handling
- Missing convenience features

### Priority 3: Low (Future Consideration)

Polish and enhancements:
- Feature requests
- UI polish
- Minor inconsistencies
- Documentation gaps

---

## Test Scenarios

### Scenario 1: Daily Workflow Test

**Duration:** 1 week
**Goal:** Use as your primary editor for routine tasks

**Steps:**
1. Replace your current editor with Notepad++ Rust for one week
2. Perform your normal editing tasks
3. Note any friction points or missing features
4. Track all issues encountered

**Success Criteria:**
- Can complete all normal tasks
- No data loss
- Acceptable performance
- No crashes during normal use

**Report:**
- Total time used: ____
- Tasks completed: ____
- Issues encountered: ____
- Would you continue using it? Yes/No/Maybe

### Scenario 2: File Operations Stress Test

**Duration:** 30 minutes
**Goal:** Test file handling under various conditions

**Test Cases:**

| Test | Steps | Expected Result |
|------|-------|----------------|
| Large file | Open 50MB text file | Opens within 5 seconds |
| Many files | Open 20 files simultaneously | All open successfully |
| Mixed encodings | Open UTF-8, UTF-16, ANSI files | Encoding detected correctly |
| Mixed EOL | Open files with Windows/Unix/Mac line endings | EOL format detected |
| Binary file | Try opening .exe or .dll | Graceful error or opens safely |
| Read-only file | Open locked file | Opens as read-only |
| Network file | Open file from network share | Opens correctly |
| Long path | File with path >260 characters | Opens successfully (Windows) |

**Record:**
- ✅ Pass / ❌ Fail for each test
- Time to open large file: ____
- Memory usage with 20 files: ____
- Any crashes or errors: ____

### Scenario 3: Editing Features Test

**Duration:** 20 minutes
**Goal:** Verify all editing operations work correctly

**Test Matrix:**

**Basic Operations:**
- [ ] Type text
- [ ] Delete text (Backspace, Delete)
- [ ] Select text (mouse, keyboard, Ctrl+A)
- [ ] Copy/Cut/Paste
- [ ] Undo/Redo (single, multiple levels)
- [ ] Drag and drop text

**Advanced Operations:**
- [ ] Multi-line selection
- [ ] Rectangle selection (Planned - skip if not available)
- [ ] Line duplication (`Ctrl+D`)
- [ ] Line deletion (`Ctrl+L`)
- [ ] Line swap (`Ctrl+T`)
- [ ] Indent/Unindent
- [ ] Auto-indent on new line

**Unicode Support:**
- [ ] Type emoji: 😀 🦀 ✨
- [ ] Type Chinese: 你好世界
- [ ] Type Arabic: مرحبا
- [ ] Type special symbols: © ® ™ € £ ¥

### Scenario 4: Search and Replace Test

**Duration:** 25 minutes
**Goal:** Test all search/replace features

**Test File:** Create file with:
```
The quick brown fox jumps over the lazy dog.
THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.
123-456-7890
test@example.com
http://example.com
Error: line 42
Warning: line 105
```

**Test Cases:**

1. **Basic Search:**
   - Search for "fox" → Found on line 1 and 2
   - Search for "FOX" with case-sensitive → Only line 2
   - Search for "cat" → Not found

2. **Whole Word:**
   - Search "he" whole word OFF → Finds "The", "the"
   - Search "he" whole word ON → Not found
   - Search "dog" whole word ON → Found

3. **Regular Expression:**
   - `\d{3}-\d{3}-\d{4}` → Finds phone number
   - `\w+@\w+\.\w+` → Finds email
   - `(Error|Warning): line \d+` → Finds both errors
   - `^THE` → Finds line starting with "THE"
   - `dog\.$` → Finds line ending with "dog."

4. **Replace:**
   - Replace "fox" with "cat" → First occurrence
   - Replace all "the" with "a" (case-insensitive) → All occurrences
   - Replace `(\d{3})-(\d{3})-(\d{4})` with `($1) $2-$3` → Format phone

**Record Results:**
- All searches work: ✅/❌
- Regex patterns work: ✅/❌
- Replace operations work: ✅/❌
- Any unexpected behavior: ____

### Scenario 5: Configuration Test

**Duration:** 15 minutes
**Goal:** Test all settings

**Test Procedure:**

1. **Tab Settings:**
   - Change tab size: 2, 4, 8
   - Toggle tabs/spaces
   - Verify in new file

2. **EOL Settings:**
   - Change default to Windows
   - Create new file → Save → Check with hex editor
   - Repeat for Unix and Mac

3. **Encoding:**
   - Set default to UTF-8
   - Create file with emoji → Save → Reopen
   - Repeat with UTF-16

4. **Backup:**
   - Enable backup
   - Edit and save file
   - Check backup directory for backup file
   - Verify backup contains previous version

5. **Session:**
   - Enable "Remember session"
   - Open 3 files
   - Close application
   - Reopen → Verify files restored

**Record:**
- Settings persist after restart: ✅/❌
- Settings apply correctly: ✅/❌
- Backup works: ✅/❌
- Session restore works: ✅/❌

### Scenario 6: Performance Test

**Duration:** 30 minutes
**Goal:** Measure performance metrics

**Test Setup:**
Create test files:
- Small: 1 KB (100 lines)
- Medium: 1 MB (20,000 lines)
- Large: 10 MB (200,000 lines)
- Huge: 50 MB (1,000,000 lines)

**Metrics to Record:**

| Operation | Small | Medium | Large | Huge |
|-----------|-------|--------|-------|------|
| Open time (s) | | | | |
| Save time (s) | | | | |
| Search time (s) | | | | |
| Scroll to end (s) | | | | |
| Memory usage (MB) | | | | |
| Replace all (s) | | | | |

**Tools:**
- Use Stopwatch for timing
- Task Manager for memory
- Generate test files with script:
```bash
# Windows PowerShell
1..1000000 | ForEach-Object { "Line $_: The quick brown fox jumps over the lazy dog." } | Out-File -FilePath huge.txt
```

**Acceptance Criteria:**
- Large file (10MB) opens < 2 seconds
- Search in large file < 1 second
- Memory usage reasonable (< 2x file size)
- No lag when scrolling

### Scenario 7: Edge Cases and Error Handling

**Duration:** 20 minutes
**Goal:** Test unusual conditions

**Test Cases:**

1. **Empty Files:**
   - Create empty file → Save → Reopen
   - ✅ Works without errors

2. **Very Long Lines:**
   - Create file with 10,000 character line
   - ✅ Displays without crashing

3. **Special Characters:**
   - Filenames with spaces, dots, unicode
   - ✅ Opens and saves correctly

4. **Disk Full:**
   - Fill disk to capacity
   - Try to save file
   - ✅ Error message shown, no crash

5. **File Deleted While Open:**
   - Open file → Delete it externally → Try to save
   - ✅ Appropriate error or prompt

6. **File Modified Externally:**
   - Open file → Modify externally → Switch back
   - ✅ Reload prompt appears

7. **Crash Recovery:**
   - Force crash (simulate)
   - Relaunch application
   - ✅ Recovery dialog appears (if implemented)

---

## Reporting Issues

### Before Reporting

1. **Search Existing Issues:**
   - Check GitHub issues: `is:issue label:bug`
   - May already be reported

2. **Reproduce the Issue:**
   - Can you make it happen again?
   - What are the exact steps?

3. **Collect Information:**
   - Error messages
   - Screenshots
   - Log files
   - System information

### Bug Report Template

When creating a new issue, include:

```markdown
## Bug Description
Clear, concise description of what went wrong.

## Steps to Reproduce
1. Open application
2. Click File → Open
3. Select file "test.txt"
4. Click button XYZ
5. Observe error

## Expected Behavior
What should have happened?

## Actual Behavior
What actually happened?

## Screenshots
[Attach screenshots if applicable]

## Environment
- OS: Windows 11 23H2
- Notepad++ Rust Version: 8.0.0-beta.1
- File size: 1.5 MB
- File encoding: UTF-8

## Logs
```
[Paste relevant log entries from %APPDATA%\Notepad++ Rust\logs\]
```

## Additional Context
Any other relevant information
```

### Severity Guidelines

**Critical (P0):**
- Application won't launch
- Crashes on startup
- Cannot open files
- Cannot save files
- Data loss

**High (P1):**
- Frequent crashes
- Major features broken
- Severe performance issues
- Data corruption (rare)

**Medium (P2):**
- Occasional crashes
- Minor features not working
- UI glitches
- Moderate performance issues

**Low (P3):**
- Cosmetic issues
- Rare edge cases
- Feature requests
- Documentation errors

### Feature Requests

Use the Feature Request template:

```markdown
## Feature Description
What feature would you like to see?

## Use Case
Why is this feature needed? What problem does it solve?

## Proposed Solution
How do you envision this working?

## Alternatives Considered
What alternatives have you considered?

## Priority
How important is this to you? (Critical/High/Medium/Low)
```

---

## Known Issues

### Current Known Bugs (Beta 1)

**Integration tests don't run on Linux:**
- Status: Known limitation
- Workaround: Tests are Windows-only currently
- Fix ETA: Phase 7 (Linux support)

**Missing Features:**
- Multi-tab interface (Planned: Phase 5)
- Syntax highlighting (Planned: Phase 5)
- Plugin system (Planned: Phase 6)
- Macros (Planned: Phase 5)
- Split view (Planned: Phase 5)

**Performance:**
- Large files (>100MB) may be slow
- Workaround: Use stream mode (future)

### Won't Fix (By Design)

- Plugin compatibility with original Notepad++: Requires rewrite
- Identical UI to original: Some differences expected
- Windows XP/7 support: Windows 10+ only

---

## Feedback Channels

### GitHub Issues
**Best for:** Bug reports, feature requests
**URL:** https://github.com/yourusername/notepad-plus-rust/issues

**Labels:**
- `bug` - Something isn't working
- `enhancement` - New feature or improvement
- `beta-feedback` - General beta testing feedback
- `documentation` - Documentation issues
- `performance` - Performance problems
- `P0-critical`, `P1-high`, `P2-medium`, `P3-low` - Priority

### GitHub Discussions
**Best for:** Questions, ideas, general feedback
**URL:** https://github.com/yourusername/notepad-plus-rust/discussions

**Categories:**
- General - General discussion
- Ideas - Feature ideas and suggestions
- Q&A - Questions about usage
- Beta Program - Beta testing discussion

### Discord
**Best for:** Real-time chat, quick questions
**URL:** https://discord.gg/notepad-plus-rust

**Channels:**
- #beta-testers - Beta program chat
- #bug-reports - Quick bug reports
- #feedback - General feedback
- #help - Get help with issues

### Email
**Best for:** Private/security issues
**Email:** beta@notepad-plus-rust.org

---

## Beta Testing Checklist

### Week 1: Initial Testing

- [ ] Install beta build
- [ ] Complete smoke test
- [ ] Daily workflow test (use for all editing)
- [ ] File operations stress test
- [ ] Report at least 3 pieces of feedback

### Week 2: Focused Testing

- [ ] Editing features test
- [ ] Search and replace test
- [ ] Configuration test
- [ ] Performance benchmarks
- [ ] Retest any fixed bugs

### Week 3: Final Validation

- [ ] Install Release Candidate
- [ ] Verify all P0/P1 bugs fixed
- [ ] Stability test (use heavily for 3 days)
- [ ] Write final feedback summary
- [ ] Complete beta survey

### Beta Survey (End of Week 3)

**Please complete our survey:**
https://forms.notepad-plus-rust.org/beta-survey

**Questions include:**
- Overall satisfaction (1-10)
- Would you recommend it?
- Most useful features
- Most missed features
- Biggest issues encountered
- Performance rating
- Stability rating
- Would you switch from your current editor?

---

## Rewards and Recognition

### Beta Tester Benefits

- 🎖️ **Beta Tester badge** on GitHub/Discord
- 📝 **Credit in release notes** (with permission)
- 🎁 **Swag** for top contributors (t-shirt, stickers)
- 🌟 **Early access** to future betas
- 💬 **Direct line** to development team

### Top Contributor Criteria

Determined by:
- Quality of bug reports
- Number of issues found
- Severity of issues found
- Feedback quality and detail
- Community participation

**Top 10 contributors will receive:**
- Notepad++ Rust Edition t-shirt
- Sticker pack
- Permanent "Beta Hero" badge
- Mentioned in launch announcement

---

## Thank You!

Your participation in the Notepad++ Rust Edition beta program is invaluable. Every bug report, feature suggestion, and piece of feedback helps us build a better editor.

Happy testing! 🦀

---

**Beta Program Contacts:**
- Program Manager: beta-manager@notepad-plus-rust.org
- Technical Lead: tech@notepad-plus-rust.org
- Community Manager: community@notepad-plus-rust.org

**Beta Build:**
- Current Version: 8.0.0-beta.1
- Build Date: 2024-03-17
- Next Update: Weekly on Fridays
