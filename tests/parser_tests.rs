use vnext::models::commit::CommitParser;

// Default commit types for testing
fn default_major_types() -> Vec<&'static str> {
    vec!["major"]
}

fn default_minor_types() -> Vec<&'static str> {
    vec!["feat", "minor"]
}

fn default_noop_types() -> Vec<&'static str> {
    vec!["chore", "noop"]
}
use vnext::parsers::{
    ConventionalCommitParser,
    CustomRegexParser,
    ParserFactory,
    ParserStrategy,
    MAJOR_REGEX_STR,
    MINOR_REGEX_STR,
    NOOP_REGEX_STR,
    BREAKING_REGEX_STR,
    TYPE_REGEX_STR,
    SCOPE_REGEX_STR,
};

#[test]
fn test_conventional_commit_parser() {
    let parser = ConventionalCommitParser::new();
    
    // Test major changes
    let major_commit1 = parser.parse_commit("test1".to_string(), "feat!: Breaking change".to_string());
    let major_commit2 = parser.parse_commit("test2".to_string(), "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.".to_string());
    let major_commit3 = parser.parse_commit("test3".to_string(), "major: This is a major change".to_string());
    
    assert!(major_commit1.is_major_change(&default_major_types()));
    assert!(major_commit2.is_major_change(&default_major_types()));
    assert!(major_commit3.is_major_change(&default_major_types()));
    
    // Test minor changes
    let minor_commit1 = parser.parse_commit("test4".to_string(), "feat: Add new feature".to_string());
    let minor_commit2 = parser.parse_commit("test5".to_string(), "minor: This is a minor change".to_string());
    let not_minor_commit = parser.parse_commit("test6".to_string(), "fix: Fix a bug".to_string());
    
    assert!(minor_commit1.is_minor_change(&default_minor_types()));
    assert!(minor_commit2.is_minor_change(&default_minor_types()));
    assert!(!not_minor_commit.is_minor_change(&default_minor_types()));
    
    // Test no-op changes
    let noop_commit1 = parser.parse_commit("test7".to_string(), "chore: Update dependencies".to_string());
    let noop_commit2 = parser.parse_commit("test8".to_string(), "noop: This is a no-op change".to_string());
    let not_noop_commit = parser.parse_commit("test9".to_string(), "feat: Add new feature".to_string());
    
    assert!(noop_commit1.is_noop_change(&default_noop_types()));
    assert!(noop_commit2.is_noop_change(&default_noop_types()));
    assert!(!not_noop_commit.is_noop_change(&default_noop_types()));
    
    // Test breaking changes
    let breaking_commit1 = parser.parse_commit("test10".to_string(), "feat!: Breaking change".to_string());
    let breaking_commit2 = parser.parse_commit("test11".to_string(), "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.".to_string());
    let not_breaking_commit = parser.parse_commit("test12".to_string(), "feat: Add new feature".to_string());
    
    assert!(breaking_commit1.has_breaking_change);
    assert!(breaking_commit2.has_breaking_change);
    assert!(!not_breaking_commit.has_breaking_change);
    
    // Test parsing a commit
    let commit = parser.parse_commit("abc123".to_string(), "feat(ui): Add new button\n\nThis adds a new button to the UI.".to_string());
    assert_eq!(commit.commit_id, "abc123");
    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, Some("ui".to_string()));
    assert_eq!(commit.title, "Add new button");
    assert_eq!(commit.body, Some("This adds a new button to the UI.".to_string()));
    assert!(!commit.has_breaking_change);
}

#[test]
fn test_custom_regex_parser() {
    let parser = CustomRegexParser::default();
    
    // Test major changes
    let major_commit1 = parser.parse_commit("test1".to_string(), "major: This is a major change".to_string());
    let major_commit2 = parser.parse_commit("test2".to_string(), "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.".to_string());
    let not_major_commit = parser.parse_commit("test3".to_string(), "feat: Add new feature".to_string());
    
    assert!(major_commit1.is_major_change(&default_major_types()));
    assert!(major_commit2.is_major_change(&default_major_types()));
    assert!(!not_major_commit.is_major_change(&default_major_types()));
    
    // Test minor changes
    let minor_commit1 = parser.parse_commit("test4".to_string(), "feat: Add new feature".to_string());
    let minor_commit2 = parser.parse_commit("test5".to_string(), "minor: This is a minor change".to_string());
    let not_minor_commit = parser.parse_commit("test6".to_string(), "fix: Fix a bug".to_string());
    
    assert!(minor_commit1.is_minor_change(&default_minor_types()));
    assert!(minor_commit2.is_minor_change(&default_minor_types()));
    assert!(!not_minor_commit.is_minor_change(&default_minor_types()));
    
    // Test no-op changes
    let noop_commit1 = parser.parse_commit("test7".to_string(), "chore: Update dependencies".to_string());
    let noop_commit2 = parser.parse_commit("test8".to_string(), "noop: This is a no-op change".to_string());
    let not_noop_commit = parser.parse_commit("test9".to_string(), "feat: Add new feature".to_string());
    
    assert!(noop_commit1.is_noop_change(&default_noop_types()));
    assert!(noop_commit2.is_noop_change(&default_noop_types()));
    assert!(!not_noop_commit.is_noop_change(&default_noop_types()));
    
    // Test breaking changes
    let breaking_commit = parser.parse_commit("test10".to_string(), "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.".to_string());
    let not_breaking_commit = parser.parse_commit("test11".to_string(), "feat: Add new feature".to_string());
    
    assert!(breaking_commit.has_breaking_change);
    assert!(!not_breaking_commit.has_breaking_change);
    
    // Test parsing a commit
    let commit = parser.parse_commit("abc123".to_string(), "feat(ui): Add new button\n\nThis adds a new button to the UI.".to_string());
    assert_eq!(commit.commit_id, "abc123");
    assert_eq!(commit.commit_type, "feat");
    assert_eq!(commit.scope, Some("ui".to_string()));
    assert_eq!(commit.title, "Add new button");
    assert!(commit.body.is_some());
    
    // Test with custom patterns
    let custom_parser = CustomRegexParser::new(
        r"(?m)^custom-major:.*",
        r"(?m)^custom-minor:.*",
        r"(?m)^custom-noop:.*",
        r"(?m)^custom-breaking:.*",
        r"^custom-([\w-]+):",  // Extract "major", "minor", etc. from "custom-major", "custom-minor"
        r"^custom-[\w-]+\((.*)\):",  // Custom scope pattern (likely won't match in these tests)
    ).unwrap();
    
    let custom_major = custom_parser.parse_commit("test12".to_string(), "custom-major: This is a major change".to_string());
    let not_custom_major = custom_parser.parse_commit("test13".to_string(), "major: This is a major change".to_string());
    
    let custom_minor = custom_parser.parse_commit("test14".to_string(), "custom-minor: This is a minor change".to_string());
    let not_custom_minor = custom_parser.parse_commit("test15".to_string(), "minor: This is a minor change".to_string());
    
    let custom_noop = custom_parser.parse_commit("test16".to_string(), "custom-noop: This is a no-op change".to_string());
    let not_custom_noop = custom_parser.parse_commit("test17".to_string(), "noop: This is a no-op change".to_string());
    
    let custom_breaking = custom_parser.parse_commit("test18".to_string(), "custom-breaking: This is a breaking change".to_string());
    let not_custom_breaking = custom_parser.parse_commit("test19".to_string(), "BREAKING CHANGE: This is a breaking change".to_string());
    
    assert!(custom_major.is_major_change(&default_major_types()));
    assert!(!not_custom_major.is_major_change(&default_major_types()));
    
    assert!(custom_minor.is_minor_change(&default_minor_types()));
    assert!(!not_custom_minor.is_minor_change(&default_minor_types()));
    
    assert!(custom_noop.is_noop_change(&default_noop_types()));
    assert!(!not_custom_noop.is_noop_change(&default_noop_types()));
    
    assert!(custom_breaking.has_breaking_change);
    assert!(!not_custom_breaking.has_breaking_change);
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
        type_pattern: TYPE_REGEX_STR.to_string(),
        scope_pattern: SCOPE_REGEX_STR.to_string(),
    });
    assert_eq!(custom_parser.name(), "custom-regex");
    
    // Test with invalid regex patterns (should fall back to defaults)
    let invalid_parser = ParserFactory::create(&ParserStrategy::CustomRegex {
        major_pattern: "[invalid regex".to_string(),
        minor_pattern: MINOR_REGEX_STR.to_string(),
        noop_pattern: NOOP_REGEX_STR.to_string(),
        breaking_pattern: BREAKING_REGEX_STR.to_string(),
        type_pattern: TYPE_REGEX_STR.to_string(),
        scope_pattern: SCOPE_REGEX_STR.to_string(),
    });
    assert_eq!(invalid_parser.name(), "custom-regex");
    
    // Test parsing with different parsers
    let commit_message = "feat: Add new feature";
    
    let conv_commit = conventional_parser.parse_commit("test20".to_string(), commit_message.to_string());
    let custom_commit = custom_parser.parse_commit("test21".to_string(), commit_message.to_string());
    
    assert!(conv_commit.is_minor_change(&default_minor_types()));
    assert!(custom_commit.is_minor_change(&default_minor_types()));
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
        type_pattern: TYPE_REGEX_STR.to_string(),
        scope_pattern: SCOPE_REGEX_STR.to_string(),
    });
    
    // Test with a major change
    let major_message = "feat!: Breaking change";
    let conv_major = conventional_parser.parse_commit("test22".to_string(), major_message.to_string());
    let custom_major = custom_parser.parse_commit("test23".to_string(), major_message.to_string());
    
    assert!(conv_major.is_major_change(&default_major_types()));
    assert!(!custom_major.is_major_change(&default_major_types())); // Custom parser doesn't recognize ! as breaking
    
    // Test with a minor change
    let minor_message = "feat: Add new feature";
    let conv_minor = conventional_parser.parse_commit("test24".to_string(), minor_message.to_string());
    let custom_minor = custom_parser.parse_commit("test25".to_string(), minor_message.to_string());
    
    assert!(conv_minor.is_minor_change(&default_minor_types()));
    assert!(custom_minor.is_minor_change(&default_minor_types()));
    
    // Test with a patch change
    let patch_message = "fix: Fix a bug";
    let conv_patch = conventional_parser.parse_commit("test26".to_string(), patch_message.to_string());
    let custom_patch = custom_parser.parse_commit("test27".to_string(), patch_message.to_string());
    
    assert!(!conv_patch.is_major_change(&default_major_types()));
    assert!(!conv_patch.is_minor_change(&default_minor_types()));
    assert!(!conv_patch.is_noop_change(&default_noop_types()));
    
    assert!(!custom_patch.is_major_change(&default_major_types()));
    assert!(!custom_patch.is_minor_change(&default_minor_types()));
    assert!(!custom_patch.is_noop_change(&default_noop_types()));
    
    // Test with a no-op change
    let noop_message = "chore: Update dependencies";
    let conv_noop = conventional_parser.parse_commit("test28".to_string(), noop_message.to_string());
    let custom_noop = custom_parser.parse_commit("test29".to_string(), noop_message.to_string());
    
    assert!(conv_noop.is_noop_change(&default_noop_types()));
    assert!(custom_noop.is_noop_change(&default_noop_types()));
}