# vnext

**vnext** is a fast Rust CLI tool that analyzes your project's Git commit history using conventional commit conventions to automatically compute your next semantic version. It streamlines release management by deciding whether to bump the major, minor, or patch version solely from commit messages, making it ideal for any project regardless of language or ecosystem.

## Motivation

Version management has traditionally been a manual, subjective process prone to inconsistency and human bias. Emotionless versioning—where version numbers are determined by objective rules rather than human judgment—eliminates these issues by creating a standardized, predictable release process. This approach removes debates about what constitutes a "major" or "minor" change, prevents version stagnation due to fear of incrementing major versions, and ensures version numbers accurately reflect the nature of changes.

Beyond just determining version numbers, conventional commits serve a dual purpose: they communicate changes in plain English that can be automatically compiled into changelogs. These changelogs provide valuable documentation that is surfaced by dependency management tools like Renovate, allowing consuming developers to understand exactly what changes to expect with each version update. This creates a seamless information flow from commit message to version number to changelog to dependency update notification, enhancing transparency and trust throughout the development ecosystem.

Semantic-release is a powerful tool for automated versioning and changelog generation in the Node.js ecosystem, but it's tightly bound to Node and depends on the presence of a package.json, often introducing unnecessary overhead. While alternative tools exist, few offer the same streamlined experience. To address this gap, vnext was created—a lightweight, language-agnostic utility designed to parse Git commit messages and output the next semantic version. It adheres to the Unix philosophy of "do one thing well," making it an ideal choice for CI/CD pipelines across any tech stack.

## Features

- **Automated Version Calculation:**
   vnext scans your Git commit history (starting from the last version tag in `v*.*.*` format) and examines each commit message using predefined regular expressions. This means you can create any tag number from your current code base (e.g., `v1.2.3`) and vnext will use that as the starting point for future version calculations. It follows these rules:

   - **Major Version:**
      If a commit message is marked as a major change (with `major:`) or contains `BREAKING CHANGE:` at the start of the first line of the commit body, vnext triggers a major version bump. This resets the minor and patch numbers. Note that `BREAKING CHANGE:` must appear at the beginning of the first line of the commit body (right after the commit title and an empty line) to trigger a major version bump.
      
      **Example of a commit message that triggers a major version bump:**
      ```
      feat: add new authentication system
      
      BREAKING CHANGE: Users will need to re-authenticate after this update
      ```
      
      **Examples that will NOT trigger a major version bump:**
      ```
      feat: add new feature
      
      This feature includes BREAKING CHANGE: in the middle of a line
      ```
      
      ```
      feat: add new feature
      
      This is the first line of the body.
      BREAKING CHANGE: This is not the first line of the body
      ```

   - **Minor Version:**  
      In the absence of major changes, if any commit message signals the introduction of a new feature—typically using the `feat:` prefix—vnext bumps the minor version and resets the patch number.

   - **Patch Version:**  
      If neither major nor minor changes are detected, but there’s at least one commit marked as a bug fix (usually indicated by `fix:`), vnext increments the patch version.

   - **No-Ops:**  
      Some commits (such as those labeled with `chore:`, `noop:`, or other non-functional changes) are ignored in the version calculation. These commits are treated as no-ops and do not trigger any version bump.

   **Note on Manual Version Bumps:**
   The prefixes like `major:`, `minor:`, and `noop:` are considered "escape hatches" and not technically semantic versioning based on the semantics of the changes. They're included for convenience when you need to explicitly control version bumps outside the standard conventional commit types. While they're handy tools, relying primarily on semantic commit types (`feat:`, `fix:`, etc.) and `BREAKING CHANGE:` notations is more aligned with true semantic versioning principles.

   This simple yet robust mechanism makes vnext ideal for integrating into CI/CD pipelines, regardless of the project's language or ecosystem.
- **Language-Agnostic:**  
  No Node.js, npm, or package.json required – works with any codebase - it's all based on git.
- **Simplicity First:**  
  Outputs just the next semantic version. Combine it with your preferred release process.
- **Unix Philosophy:**  
  A small, focused tool that does one thing well.

## Sequence Diagram(s)

```mermaid
sequenceDiagram
    participant User
    participant CLI
    participant Git
    participant VersionLogic

    User->>CLI: Run versioning tool
    CLI->>Git: Check for previous release tags
    alt Tags exist
        CLI->>Git: Find merge base between HEAD and tag
        Git-->>CLI: Return base commit
    else No tags
        CLI->>Git: Traverse first-parent chain to initial commit
        Git-->>CLI: Return initial commit as base
    end
    CLI->>VersionLogic: Analyze commits since base commit
    VersionLogic->>Git: Get commit list
    loop For each commit
        VersionLogic->>VersionLogic: Classify and record commit (including no-ops)
    end
    VersionLogic-->>CLI: Return version bump and summary
    CLI-->>User: Output version/changelog
```

## Installation

### Using ubi

1. **Install ubi:**  
   Ensure you have ubi installed by running:
   ```bash
   mkdir -p ~/.ubi/bin
   echo 'export PATH="$HOME/.ubi/bin:$PATH"' >> ~/.zshrc  # or your preferred shell profile
   ```
2. **Install vnext with ubi:**  
   ```bash
   ubi --project unbounded-tech/vnext --in ~/.ubi/bin
   ```

Install a specific version:

```bash
ubi --project unbounded-tech/vnext --tag v1.7.2 --in /usr/local/bin/
```

See "Releases" for available versions and changenotes.

### Building from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/unbounded-tech/vnext.git
   cd vnext
   ```
2. **Build the project:**
   ```bash
   # Standard build (uses system OpenSSL)
   cargo build --release
   
   # Build with vendored OpenSSL (standalone binary)
   cargo build --release --features vendored
   ```
   The compiled binary will be located at `target/release/vnext`.

   > **Note:** The `vendored` feature statically links OpenSSL, creating a standalone binary that works on systems without OpenSSL installed. This is recommended for distribution but increases build time.

## Usage

After installation or building from source, run:
```bash
vnext help
```
To compute the next version based on your Git commit history, simply run:
```bash
vnext
```
This will output the new semantic version, ready for use in your release pipelines.

### Getting the Current Version

To get the current version that vnext is bumping from, use the `--current` flag:

```bash
vnext --current
```

This will output the current version (the version of the latest tag, or 0.0.0 if no tags exist).

### Generating a Changelog

To generate a changelog based on your commit history, use the `--changelog` flag:

```bash
vnext --changelog
```

This will output a formatted changelog that includes all commits since the last version tag, organized by their impact on versioning. For example:

```
### What's changed in v1.2.0

* feat: add new authentication system
* fix: resolve login issue with special characters
* chore: update dependencies
```

#### Header Scaling in Changelogs

By default, vnext automatically scales down markdown headers in commit bodies to maintain a consistent visual hierarchy in the generated changelog. This is particularly useful when the changelog is displayed in GitHub release notes, where the "What's changed" header is already an H3.

The scaling works as follows:
- H1 (#) → H4 (####)
- H2 (##) → H5 (#####)
- H3 (###) → H6 (######)

Headers H4 and below remain unchanged.

To disable header scaling and preserve the original header levels, use the `--no-header-scaling` flag:

```bash
vnext --changelog --no-header-scaling
```

#### GitHub Contributor Information

When the repository is hosted on GitHub (detected by having a remote with "github" in the URL), GitHub contributor information is automatically included in the changelog when using `--changelog`.

This will add the GitHub username of the commit author to each entry in the changelog:

```
### What's changed in v1.2.0

* feat: add new authentication system (by @johndoe)
* fix: resolve login issue with special characters (by @janedoe)
* chore: update dependencies (by @devuser)
```

For private repositories, you'll need to set the `GITHUB_TOKEN` environment variable with a valid GitHub personal access token:

```bash
export GITHUB_TOKEN=your_github_token
vnext --changelog
```

This is set automatically when using Github Actions.

The changelog includes the commit messages and preserves multi-line commit bodies with proper indentation:

```
### What's changed in v2.0.0

* feat: redesign user interface

  This commit completely overhauls the UI with a new design system.
  - Improved accessibility
  - Better mobile support

* fix: resolve performance issues in data processing
```

For breaking changes, the changelog will include the breaking change notice:

```
### What's changed in v3.0.0

* feat: migrate to new API

  BREAKING CHANGE: This removes support for the legacy API endpoints
```

This flag is particularly useful in CI/CD pipelines to automatically generate release notes. The shared GitHub workflow at [unbounded-tech/workflow-vnext-tag](https://github.com/unbounded-tech/workflow-vnext-tag) uses this flag to generate and save a CHANGELOG.md file during the release process.

### Generating a Deploy Key for GitHub

The `generate-deploy-key` subcommand allows you to create a deploy key for a GitHub repository, which is particularly useful for CI/CD workflows:

```bash
vnext generate-deploy-key [--owner OWNER] [--name NAME] [--key-name KEY_NAME] [--overwrite]
```

If you run this command within a GitHub repository, it will automatically detect the repository owner and name and ask if you want to use them. Otherwise, it will prompt you to enter the repository information.

#### Deploy Key and Secret Management

The command checks if a deploy key or secret already exists before creating new ones:

- If neither exists, it creates both the deploy key and secret
- If either exists and `--overwrite` is not specified, it prompts for confirmation
- If either exists and `--overwrite` is specified, it replaces them without prompting
- If both exist and `--overwrite` is not specified or denied, it skips creation

#### Why Deploy Keys Are Necessary

When using GitHub Actions, the default `GITHUB_TOKEN` provided to workflows has a critical limitation: **it cannot trigger other workflows**. This is a security measure to prevent accidental workflow loops, but it means that if you have a workflow that creates a tag (like the vnext tagging workflow), that tag creation won't trigger any release workflows you might have set up.

To overcome this limitation, you have two options:
1. Use a Personal Access Token (PAT) with appropriate permissions - For paid organization, you can use an organization wide secret for this effectively.
2. Use a deploy key (SSH key) specifically for this repository - especially useful for free github orgs.

The `generate-deploy-key` command simplifies the second option by:
1. Generating an Ed25519 SSH key pair
2. Setting up the private key as a GitHub repository secret named `DEPLOY_KEY` (or a custom name you specify)
3. Adding the public key as a deploy key to the repository

#### Using with the Shared Workflow

The shared workflow at [unbounded-tech/workflow-vnext-tag](https://github.com/unbounded-tech/workflow-vnext-tag) supports using deploy keys for tag creation. To use it:

1. Generate a deploy key for your repository:
   ```bash
   vnext generate-deploy-key
   ```

2. In your workflow file, enable the deploy key option:
   ```yaml
   version-and-tag:
     uses: unbounded-tech/workflow-vnext-tag/.github/workflows/workflow.yaml@main
     secrets: inherit
     with:
       useDeployKey: true
   ```

This ensures that when the workflow creates a tag, it will use the deploy key instead of the default `GITHUB_TOKEN`, allowing the tag to trigger other workflows like releases.

Humans do not need to know this key and secret. You can simply rotate the key by using the `--overwrite` flag:

```bash
# Rotate the deploy key by generating a new one
vnext generate-deploy-key --overwrite
```

This will replace the existing key and secret with a new pair, which is useful for security best practices that recommend periodic key rotation.

### Starting from a Specific Version

If you want to start versioning from a specific version number, simply create a Git tag with that version:

```bash
# Tag the current commit with a specific version
git tag v2.5.0

# Future runs of vnext will use this as the starting point
vnext  # Might output 2.5.1, 2.6.0, or 3.0.0 depending on commits since the tag
```

This allows you to initialize your versioning at any point, which is especially useful when adopting vnext in an existing project.

## Developer Guide

### Building and Running

1. **Clone and Build:**
   ```bash
   git clone https://github.com/unbounded-tech/vnext.git
   cd vnext
   
   # For development (faster build)
   cargo build
   
   # For release with vendored OpenSSL
   cargo build --release --features vendored
   ```
2. **Run the Tool:**
   ```bash
   cargo run -- help
   ```
3. **Debugging:**
   Use the provided VSCode configuration in `.vscode/launch.json` to launch the debugger with breakpoints.

## Logging

vnext uses structured, colored logging similar to Cargo's output. By default, the log level is set to `info`. For more detailed logs, set:
```bash
export LOG_LEVEL=debug
```

## GitHub Actions

### Shared Workflow

A shared github workflow can be found at [https://github.com/unbounded-tech/workflow-vnext-tag](unbounded-tech/workflow-vnext-tag).

Generally, you can just use this workflow. You can then trigger a release workflow on when tags are created.

You can check out this repo's `.github` folder for more robust examples, but here's the basics:

Example for rust:

```
on:
  push:
    branches:
      - main

jobs:
  quality:
    uses: unbounded-tech/workflow-rust-quality/.github/workflows/workflow.yaml@main
    with:
      cargo_test_args: '--verbose'
      lint: true

  version-and-tag:
    needs: quality
    uses: unbounded-tech/workflow-vnext-tag/.github/workflows/workflow.yaml@main
    secrets: inherit
    with:
      useDeployKey: true
      rust: true
```

This will create a `v*.*.*` tag, which you can use to trigger other workflows:

```
name: On Version Tagged, Build and Publish Rust Binaries
on:
  push:
    tags:
    - "v*.*.*"

permissions:
  contents: write

jobs:
  release:
    uses: unbounded-tech/workflows-rust/.github/workflows/release.yaml@v1.2.1
    with:
      binary_name: ${{ github.event.repository.name }}
      build_args: "--release --features vendored"
```

The workflow itself uses a wrapped github action - you can find a link to that in the "Packages" section of the repo home page.

## Contributing

Contributions are welcome! Please fork the repository and open a pull request for any enhancements, bug fixes, or new features. For major changes, open an issue first to discuss your ideas.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

