### What's changed in v1.2.0

* feat: enable github integration when repo has github remote (#47)

  Automatically enables the GitHub contributor information in changelogs when:
  1. The repository is hosted on GitHub (detected from remote URL)
  2. The --changelog flag is enabled

  This improves the user experience by providing GitHub contributor information without requiring the --github flag to be explicitly set for GitHub repositories.

  - Modified main.rs to detect GitHub repositories and auto-enable the flag
  - Added tests for GitHub repository detection
  - Updated documentation to reflect the new behavior
  - Made GitHub module and extract_repo_info function public in the library
