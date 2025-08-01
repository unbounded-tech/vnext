### What's changed in v1.0.0

* fix(deps): update rust crate clap to v4.5.42 (#41)

* feat(formatting): indent commit bodies (#40)
# Improved Changelog Formatting
## Feature Description
The changelog formatting has been enhanced to provide better readability for commit messages with multi-line bodies. This improvement is particularly valuable for complex commits such as squashed PRs from GitHub UI that contain multiple bullet points with extra lines between them.
## Key Improvements
1. **Proper Indentation**: All lines in commit bodies are now indented with two spaces, creating a clear visual hierarchy between commit titles and their descriptions.
2. **Empty Line Separation**: An empty line is added between the commit title and its body, providing clear visual separation.
3. **Preserved Structure**: Empty lines within commit bodies are maintained to preserve the original structure and formatting of the message.
4. **Optimized for Squashed PRs**: The formatting is optimized to handle GitHub UI squashed PRs, which typically include bullet points with extra lines between them.
## Before and After Example
### Before:

### After:

## Implementation Details
The improved formatting is implemented in the  method in . The method now:
1. Adds a bullet point to the first line of each commit message
2. Adds an empty line between the commit title and body
3. Indents all non-empty lines in the body with two spaces
4. Preserves empty lines within the body to maintain the original structure
5. Skips leading empty lines in the body to avoid extra newlines
This enhancement makes changelogs more readable and visually appealing, especially for projects that use conventional commits and follow semantic versioning practices.
* fix(deps): update rust crate clap to v4.5.42
---------
Co-authored-by: Patrick Lee Scott <pat@patscott.io>
Co-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>
