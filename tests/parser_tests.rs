use vnext::models::commit::CommitParser;
use vnext::parsers::{
    ConventionalCommitParser,
    CustomRegexParser,
    ParserFactory,
    ParserStrategy,
    MAJOR_REGEX_STR,
    MINOR_REGEX_STR,
    NOOP_REGEX_STR,
    BREAKING_REGEX_STR,
};

#[test]
fn test_conventional_commit_parser() {
    let parser = ConventionalCommitParser::new();
    
    // Test major changes
    assert!(parser.is_major_change("feat!: Breaking change"));
    assert!(parser.is_major_change("feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API."));
    assert!(parser.is_major_change("major: This is a major change"));
    
    // Test minor changes
    assert!(parser.is_minor_change("feat: Add new feature"));
    assert!(parser.is_minor_change("minor: This is a minor change"));
    assert!(!parser.is_minor_change("fix: Fix a bug"));
    
    // Test no-op changes
    assert!(parser.is_noop_change("chore: Update dependencies"));
    assert!(parser.is_noop_change("noop: This is a no-op change"));
    assert!(!parser.is_noop_change("feat: Add new feature"));
    
    // Test breaking changes
    assert!(parser.is_breaking_change("feat!: Breaking change"));
    assert!(parser.is_breaking_change("feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API."));
    assert!(!parser.is_breaking_change("feat: Add new feature"));
    
    // Test parsing a commit
    let commit = parser.parse_commit("abc123".to_string(), "feat(ui): Add new button\n\nThis adds a new button to the UI.".to_string());
    assert_eq!(commit.commit_id, "abc123");
    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, Some("ui".to_string()));
    assert_eq!(commit.title, "Add new button");
    assert_eq!(commit.body, Some("This adds a new button to the UI.".to_string()));
    assert!(!commit.breaking_change_flag);
    assert!(!commit.breaking_change_body);
}

#[test]
fn test_custom_regex_parser() {
    let parser = CustomRegexParser::default();
    
    // Test major changes
    assert!(parser.is_major_change("major: This is a major change"));
    assert!(parser.is_major_change("feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API."));
    assert!(!parser.is_major_change("feat: Add new feature"));
    
    // Test minor changes
    assert!(parser.is_minor_change("feat: Add new feature"));
    assert!(parser.is_minor_change("minor: This is a minor change"));
    assert!(!parser.is_minor_change("fix: Fix a bug"));
    
    // Test no-op changes
    assert!(parser.is_noop_change("chore: Update dependencies"));
    assert!(parser.is_noop_change("noop: This is a no-op change"));
    assert!(!parser.is_noop_change("feat: Add new feature"));
    
    // Test breaking changes
    assert!(parser.is_breaking_change("feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API."));
    assert!(!parser.is_breaking_change("feat: Add new feature"));
    
    // Test parsing a commit
    let commit = parser.parse_commit("abc123".to_string(), "feat(ui): Add new button\n\nThis adds a new button to the UI.".to_string());
    assert_eq!(commit.commit_id, "abc123");
    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, Some("ui".to_string()));
    assert_eq!(commit.title, "feat(ui): Add new button");
    assert!(commit.body.is_some());
    
    // Test with custom patterns
    let custom_parser = CustomRegexParser::new(
        r"(?m)^custom-major:.*",
        r"(?m)^custom-minor:.*",
        r"(?m)^custom-noop:.*",
        r"(?m)^custom-breaking:.*",
    ).unwrap();
    
    assert!(custom_parser.is_major_change("custom-major: This is a major change"));
    assert!(!custom_parser.is_major_change("major: This is a major change"));
    
    assert!(custom_parser.is_minor_change("custom-minor: This is a minor change"));
    assert!(!custom_parser.is_minor_change("minor: This is a minor change"));
    
    assert!(custom_parser.is_noop_change("custom-noop: This is a no-op change"));
    assert!(!custom_parser.is_noop_change("noop: This is a no-op change"));
    
    assert!(custom_parser.is_breaking_change("custom-breaking: This is a breaking change"));
    assert!(!custom_parser.is_breaking_change("BREAKING CHANGE: This is a breaking change"));
}

#[test]
fn test_parser_factory() {
    // Test creating a conventional parser
    let conventional_parser = ParserFactory::create(&ParserStrategy::Conventional);
    assert_eq!(conventional_parser.name(), "conventional");
    
    // Test creating a custom regex parser
    let custom_parser = ParserFactory::create(&ParserStrategy::CustomRegex {
        major_pattern: MAJOR_REGEX_STR.to_string(),
        minor_pattern: MINOR_REGEX_STR.to_string(),
        noop_pattern: NOOP_REGEX_STR.to_string(),
        breaking_pattern: BREAKING_REGEX_STR.to_string(),
    });
    assert_eq!(custom_parser.name(), "custom-regex");
    
    // Test with invalid regex patterns (should fall back to defaults)
    let invalid_parser = ParserFactory::create(&ParserStrategy::CustomRegex {
        major_pattern: "[invalid regex".to_string(),
        minor_pattern: MINOR_REGEX_STR.to_string(),
        noop_pattern: NOOP_REGEX_STR.to_string(),
        breaking_pattern: BREAKING_REGEX_STR.to_string(),
    });
    assert_eq!(invalid_parser.name(), "custom-regex");
    
    // Test parsing with different parsers
    let commit_message = "feat: Add new feature";
    
    assert!(conventional_parser.is_minor_change(commit_message));
    assert!(custom_parser.is_minor_change(commit_message));
}

#[test]
fn test_integration_with_version_calculation() {
    // This test would ideally use a mock repository to test the integration with the version calculation logic
    // For simplicity, we'll just test that the parsers can be used with the version calculation logic
    
    let conventional_parser = ParserFactory::create(&ParserStrategy::Conventional);
    let custom_parser = ParserFactory::create(&ParserStrategy::CustomRegex {
        major_pattern: MAJOR_REGEX_STR.to_string(),
        minor_pattern: MINOR_REGEX_STR.to_string(),
        noop_pattern: NOOP_REGEX_STR.to_string(),
        breaking_pattern: BREAKING_REGEX_STR.to_string(),
    });
    
    // Test with a major change
    let major_message = "feat!: Breaking change";
    assert!(conventional_parser.is_major_change(major_message));
    assert!(!custom_parser.is_major_change(major_message)); // Custom parser doesn't recognize ! as breaking
    
    // Test with a minor change
    let minor_message = "feat: Add new feature";
    assert!(conventional_parser.is_minor_change(minor_message));
    assert!(custom_parser.is_minor_change(minor_message));
    
    // Test with a patch change
    let patch_message = "fix: Fix a bug";
    assert!(!conventional_parser.is_major_change(patch_message));
    assert!(!conventional_parser.is_minor_change(patch_message));
    assert!(!conventional_parser.is_noop_change(patch_message));
    
    assert!(!custom_parser.is_major_change(patch_message));
    assert!(!custom_parser.is_minor_change(patch_message));
    assert!(!custom_parser.is_noop_change(patch_message));
    
    // Test with a no-op change
    let noop_message = "chore: Update dependencies";
    assert!(conventional_parser.is_noop_change(noop_message));
    assert!(custom_parser.is_noop_change(noop_message));
}