### What's changed in v1.12.0

* feat: implement flexible commit parser architecture (#81) (by @patrickleet)

  Introduces a parser architecture for analyzing commit messages with multiple strategies:

  ##### Parser Strategies

  - **Conventional Commits Parser**: Implements the [Conventional Commits](https://www.conventionalcommits.org/) specification for structured commit messages
  - **Custom Regex Parser**: Enhanced with configurable regex patterns for flexible parsing

  ##### Key Improvements

  - **Factory Pattern**: Added `ParserFactory` to dynamically create appropriate parsers
  - **Extensible Design**: Easily add new parser strategies in the future
  - **Enhanced Custom Parser**: Now supports extracting commit type and scope with configurable regex patterns:
    - `--type_pattern`: Extracts commit type (e.g., "feat", "fix")
    - `--scope_pattern`: Extracts commit scope (e.g., "auth", "ui")

  This architecture provides a more flexible and maintainable approach to commit message parsing, allowing users to choose the strategy that best fits their workflow.


See full diff: [v1.11.0...v1.12.0](https://github.com/unbounded-tech/vnext/compare/v1.11.0...v1.12.0)
