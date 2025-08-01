### What's changed in v1.0.2

* fix: breaking change detection, readme updates (#43)

This commit fixes a bug in the BREAKING CHANGE detection logic where any occurrence 
of BREAKING CHANGE: in a commit message would trigger a major version bump, even 
if it appeared in the middle of a line or in a line that wasn't the first line of 
the commit body.
The fix modifies the regex pattern in constants.rs to only match BREAKING CHANGE: 
when it appears at the start of the first line of the commit body (immediately after 
the commit title and an empty line). This ensures that only intentional breaking 
changes trigger a major version bump, not mentions of breaking changes in 
documentation or other contexts.
Changes:
- Updated BREAKING_REGEX_STR in constants.rs to use a more specific pattern
- Added comprehensive test cases in tests/integration_test.rs
- Updated unit tests in git.rs to match the new behavior
- Updated README.md to clarify how BREAKING CHANGE detection works
This change maintains compatibility with conventional commits specification while 
preventing false positives that could lead to unintended major version bumps.
