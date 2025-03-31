# vnext

**vnext** is a Rust-based command-line tool that calculates your next version based on commit message formatting, specifically, conventional commit style, by default.

## Install

Install [ubi](https://github.com/houseabsolute/ubi) and create `~/.ubi/bin` and add it to your `PATH` (if you haven't already):

```
mkdir -p ~/.ubi/bin
echo 'export PATH="$HOME/.ubi/bin:$PATH"' >> ~/.zshrc
```

Use `ubi` to install:

```
export GITHUB_TOKEN=$(gh auth token)
ubi --project harmony-labs/vnext --in ~/.ubi/bin
```

## Features

- **Version Calculation:** Analyzes git commit history using conventional commit conventions (e.g., `feat:`, `fix:`, `BREAKING CHANGE`) to calculate a new version number.

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable recommended)

## Developer Guide

### Building from Source

1. **Clone the repository:**

   ```bash
   git clone https://github.com/harmony-labs/vnext.git
   cd vnext
   ```

2. **Build the project:**

   ```bash
   cargo build
   ```

   or
   
   ```bash
   cargo build --release
   ```

3. **Run the project**

   ```bash
   cargo run -- help
   ```

   The compiled binary will be located at `target/release/vnext`.

## Debug

Launch VSCode's integrated debugger. Config is included in repo. New debug targets need to be created for each command.

## Usage

After building or installing, you can run the tool with various subcommands. Run help for more info.

```
vnext help
```

## Logging

vnext features structured, colored logging. By default, the log level is set to `info`. You can adjust the log level dynamically by setting the `LOG_LEVEL` environment variable:

```bash
export LOG_LEVEL=debug
```

The logging output is designed to resemble Cargo's style, with bold and colored log level indicators.

---

## GitHub Actions & Release Automation

The project leverages GitHub Actions to automate the release process.

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have improvements, bug fixes, or new features to suggest.
