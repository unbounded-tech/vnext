use clap::Parser;
use git2::{Repository, Commit};
use semver::{Version, Prerelease, BuildMetadata};
use regex::Regex;

mod logging;

#[derive(Parser)]
#[clap(author, version, about = "Calculate the next version based on conventional commits")]
struct Cli {
    /// Regex for commits triggering a major version bump (default: "BREAKING CHANGE")
    #[clap(long, default_value = "BREAKING CHANGE")]
    major: String,

    /// Regex for commits triggering a minor version bump (default: "^feat:.*")
    #[clap(long, default_value = "^feat:.*")]
    minor: String,

    /// Regex for commits that should not trigger a version bump (default: "^chore:.*")
    #[clap(long, default_value = "^chore:.*")]
    noop: String,
}

struct VersionBump {
    major: bool,
    minor: bool,
    patch: bool,
}

struct CommitSummary {
    major: u32,
    minor: u32,
    patch: u32,
    noop: u32,
}

fn main() {
    logging::init_logging().expect("Failed to initialize logging");
    log::info!("Starting vnext...");

    let cli = Cli::parse();

    log::info!("Major bump regex: {}", cli.major);
    log::info!("Minor bump regex: {}", cli.minor);
    log::info!("No-op regex: {}", cli.noop);

    let major_re = Regex::new(&cli.major).unwrap_or_else(|e| {
        log::error!("Invalid major regex '{}': {}", cli.major, e);
        std::process::exit(1);
    });
    let minor_re = Regex::new(&cli.minor).unwrap_or_else(|e| {
        log::error!("Invalid minor regex '{}': {}", cli.minor, e);
        std::process::exit(1);
    });
    let noop_re = Regex::new(&cli.noop).unwrap_or_else(|e| {
        log::error!("Invalid noop regex '{}': {}", cli.noop, e);
        std::process::exit(1);
    });

    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => {
            log::error!("Failed to open Git repository: {}. This command must be run in a Git repository directory.", e);
            std::process::exit(1);
        }
    };

    let head = match repo.head() {
        Ok(head_ref) => match head_ref.peel_to_commit() {
            Ok(commit) => commit,
            Err(e) => {
                log::error!("Failed to resolve HEAD to a commit: {}. The repository must have at least one commit.", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            log::error!("Failed to get HEAD: {}. The repository must have at least one commit on a branch (e.g., 'main' or 'master').", e);
            std::process::exit(1);
        }
    };
    log::debug!("HEAD commit: {}", head.id());

    let main_branch = find_main_branch(&repo).expect("Failed to find main branch");
    log::debug!("Main branch detected: {}", main_branch);

    let (start_version, last_tag_commit) = match find_latest_tag(&repo) {
        Some((tag, commit)) => {
            let version = parse_version(&tag).unwrap_or_else(|_| Version::new(0, 0, 0));
            log::info!("Last release: {} at commit {}", tag, commit.id());
            (version, commit)
        }
        None => {
            log::info!("No previous release tags found, starting from 0.0.0");
            let version = Version::new(0, 0, 0);
            let parents = head.parents();
            let base_commit = if parents.count() > 0 {
                let mut earliest = head.clone();
                for parent in head.parents() {
                    earliest = parent.clone(); // Traverse to the root
                }
                earliest
            } else {
                head.clone()
            };
            (version, base_commit)
        }
    };
    log::debug!("Last tag or base commit: {}", last_tag_commit.id());

    // Determine the base commit: use merge base with main if tag exists, otherwise earliest commit
    let base_commit = if last_tag_commit.id() == head.id() && head.parents().count() == 0 {
        log::debug!("Single commit repo with no tags, analyzing all commits");
        head.parents().next().map(|c| c.clone()).unwrap_or_else(|| head.clone())
    } else if find_latest_tag(&repo).is_some() {
        let merge_base = repo.merge_base(head.id(), last_tag_commit.id())
            .expect("Failed to find merge base between HEAD and tag");
        repo.find_commit(merge_base).expect("Failed to find merge base commit")
    } else {
        last_tag_commit.clone()
    };
    log::debug!("Base commit for analysis: {}", base_commit.id());

    let (bump, summary) = calculate_version_bump(&repo, &base_commit, &head, &major_re, &minor_re, &noop_re);

    log::info!(
        "Commits pending release: {} major, {} minor, {} patch, {} noop",
        summary.major, summary.minor, summary.patch, summary.noop
    );

    let next_version = calculate_next_version(&start_version, &bump);
    log::info!("Version bump: major={}, minor={}, patch={}", bump.major, bump.minor, bump.patch);
    log::info!("Next version: {}", next_version);

    println!("{}", next_version);
}

fn find_main_branch(repo: &Repository) -> Option<String> {
    for branch in ["main", "master"] {
        if repo.find_branch(branch, git2::BranchType::Local).is_ok() {
            return Some(branch.to_string());
        }
    }
    None
}

fn find_latest_tag(repo: &Repository) -> Option<(String, Commit)> {
    let tags = repo.tag_names(None).expect("Failed to get tag names");
    let mut latest_version: Option<(String, Commit)> = None;
    let mut max_version = Version::new(0, 0, 0);

    for tag in tags.iter().flatten() {
        if let Ok(tag_ref) = repo.find_reference(&format!("refs/tags/{}", tag)) {
            if let Ok(commit) = tag_ref.peel_to_commit() {
                if let Ok(version) = parse_version(tag) {
                    if version > max_version {
                        max_version = version;
                        latest_version = Some((tag.to_string(), commit));
                    }
                }
            }
        }
    }
    latest_version
}

fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v');
    Version::parse(cleaned_tag)
}

fn calculate_version_bump(_repo: &Repository, from: &Commit, to: &Commit, major_re: &Regex, minor_re: &Regex, noop_re: &Regex) -> (VersionBump, CommitSummary) {
    let mut bump = VersionBump { major: false, minor: false, patch: false };
    let mut summary = CommitSummary { major: 0, minor: 0, patch: 0, noop: 0 };
    let mut commit_count = 0;

    let mut current = to.clone();
    let base_id = from.id();
    let mut seen = std::collections::HashSet::new();

    log::debug!("Walking commits from {} to base {}", to.id(), base_id);

    // Walk from HEAD until we reach the base commit, excluding the base itself
    while current.id() != base_id {
        if seen.contains(&current.id()) {
            break; // Avoid infinite loops
        }
        seen.insert(current.id());
        commit_count += 1;

        let message = current.message().unwrap_or("");
        log::debug!("Pending commit: {} - {}", current.id(), message.lines().next().unwrap_or(""));

        if major_re.is_match(message) {
            bump.major = true;
            summary.major += 1;
        } else if minor_re.is_match(message) {
            bump.minor = true;
            summary.minor += 1;
        } else if !noop_re.is_match(message) {
            bump.patch = true;
            summary.patch += 1;
        } else {
            summary.noop += 1;
        }

        if current.parents().count() == 0 {
            break; // Reached the root
        }
        current = current.parents().next().unwrap().clone();
    }

    log::debug!("Total commits analyzed: {}", commit_count);
    (bump, summary)
}

fn calculate_next_version(current: &Version, bump: &VersionBump) -> Version {
    let mut next = current.clone();
    next.pre = Prerelease::EMPTY;
    next.build = BuildMetadata::EMPTY;

    if bump.major {
        next.major += 1;
        next.minor = 0;
        next.patch = 0;
    } else if bump.minor {
        next.minor += 1;
        next.patch = 0;
    } else if bump.patch {
        next.patch += 1;
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use semver::Version;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("v1.2.3").unwrap(), Version::new(1, 2, 3));
        assert_eq!(parse_version("1.2.3").unwrap(), Version::new(1, 2, 3));
        assert!(parse_version("invalid").is_err());
    }

    #[test]
    fn test_calculate_next_version() {
        let base = Version::new(1, 2, 3);

        // No bump
        let bump = VersionBump { major: false, minor: false, patch: false };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 3));

        // Patch bump
        let bump = VersionBump { major: false, minor: false, patch: true };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 4));

        // Minor bump
        let bump = VersionBump { major: false, minor: true, patch: false };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 3, 0));

        // Major bump
        let bump = VersionBump { major: true, minor: false, patch: false };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(2, 0, 0));
    }

    #[test]
    fn test_calculate_version_bump() {
        // Create a temporary directory for the test repository
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path().join("test_repo");

        // Initialize a bare repository
        let repo = Repository::init_bare(&repo_path).unwrap();
        let sig = git2::Signature::now("Test", "test@example.com").unwrap();
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();

        // Create a base commit
        let base_commit_id = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[],
        ).unwrap();
        let base_commit = repo.find_commit(base_commit_id).unwrap();

        // Create a commit with a breaking change
        let to_commit_id = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "feat: add new feature\nBREAKING CHANGE: something big",
            &tree,
            &[&base_commit],
        ).unwrap();
        let to_commit = repo.find_commit(to_commit_id).unwrap();

        let major_re = Regex::new("BREAKING CHANGE").unwrap();
        let minor_re = Regex::new("^feat:.*").unwrap();
        let noop_re = Regex::new("^chore:.*").unwrap();

        let (bump, summary) = calculate_version_bump(&repo, &base_commit, &to_commit, &major_re, &minor_re, &noop_re);

        assert!(bump.major);
        assert_eq!(summary.major, 1);
        assert_eq!(summary.minor, 0);
        assert_eq!(summary.patch, 0);
        assert_eq!(summary.noop, 0);

        // Temp directory is automatically cleaned up when `temp_dir` is dropped
    }
}