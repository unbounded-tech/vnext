### What's changed in v0.13.0

* feat(formatting): indent commit bodies (#40)

The changelog formatting has been enhanced to provide better readability for commit messages with multi-line bodies. This improvement is particularly valuable for complex commits such as squashed PRs from GitHub UI that contain multiple bullet points with extra lines between them.
1. **Proper Indentation**: All lines in commit bodies are now indented with two spaces, creating a clear visual hierarchy between commit titles and their descriptions.
2. **Empty Line Separation**: An empty line is added between the commit title and its body, providing clear visual separation.
3. **Preserved Structure**: Empty lines within commit bodies are maintained to preserve the original structure and formatting of the message.
4. **Optimized for Squashed PRs**: The formatting is optimized to handle GitHub UI squashed PRs, which typically include bullet points with extra lines between them.


The improved formatting is implemented in the  method in . The method now:
1. Adds a bullet point to the first line of each commit message
2. Adds an empty line between the commit title and body
3. Indents all non-empty lines in the body with two spaces
4. Preserves empty lines within the body to maintain the original structure
5. Skips leading empty lines in the body to avoid extra newlines
This enhancement makes changelogs more readable and visually appealing, especially for projects that use conventional commits and follow semantic versioning practices.
