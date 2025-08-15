use vnext::models::commit::Commit;
use vnext::utils::regex::parse_conventional_commit;

#[test]
fn test_parse_conventional_commit() {
    // Test basic commit with type and title
    let message = "feat: Add new feature";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Add new feature");
    assert!(parsed.scope.is_none());
    assert!(!parsed.breaking_change_flag);
    assert!(parsed.body.is_none());
    assert!(!parsed.breaking_change_body);
    
    // Test commit with scope
    let message = "fix(ui): Fix button alignment";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "fix");
    assert_eq!(parsed.title, "Fix button alignment");
    assert_eq!(parsed.scope, Some("ui".to_string()));
    assert!(!parsed.breaking_change_flag);
    assert!(parsed.body.is_none());
    assert!(!parsed.breaking_change_body);
    
    // Test commit with breaking change flag
    let message = "feat!: Breaking change";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Breaking change");
    assert!(parsed.scope.is_none());
    assert!(parsed.breaking_change_flag);
    assert!(parsed.body.is_none());
    assert!(!parsed.breaking_change_body);
    
    // Test commit with scope and breaking change flag
    let message = "feat(api)!: Breaking API change";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Breaking API change");
    assert_eq!(parsed.scope, Some("api".to_string()));
    assert!(parsed.breaking_change_flag);
    assert!(parsed.body.is_none());
    assert!(!parsed.breaking_change_body);
    
    // Test commit with body
    let message = "feat: Add new feature\n\nThis is a detailed description of the feature.";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Add new feature");
    assert!(parsed.scope.is_none());
    assert!(!parsed.breaking_change_flag);
    assert_eq!(parsed.body, Some("This is a detailed description of the feature.".to_string()));
    assert!(!parsed.breaking_change_body);
    
    // Test commit with breaking change at the start of the first line of the body
    let message = "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Add new feature");
    assert!(parsed.scope.is_none());
    assert!(!parsed.breaking_change_flag);
    assert_eq!(parsed.body, Some("BREAKING CHANGE: This breaks the old API.".to_string()));
    assert!(parsed.breaking_change_body);
    
    // Test commit with breaking change not at the start of the first line of the body
    let message = "feat: Add new feature\n\nThis is the first line.\nBREAKING CHANGE: This is not at the start.";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Add new feature");
    assert!(parsed.scope.is_none());
    assert!(!parsed.breaking_change_flag);
    assert_eq!(parsed.body, Some("This is the first line.\nBREAKING CHANGE: This is not at the start.".to_string()));
    assert!(!parsed.breaking_change_body);
    
    // Test commit with breaking change in the middle of a line
    let message = "feat: Add new feature\n\nThis line has BREAKING CHANGE: in the middle.";
    let parsed = parse_conventional_commit(message).unwrap();
    assert_eq!(parsed.commit_type, "feat");
    assert_eq!(parsed.title, "Add new feature");
    assert!(parsed.scope.is_none());
    assert!(!parsed.breaking_change_flag);
    assert_eq!(parsed.body, Some("This line has BREAKING CHANGE: in the middle.".to_string()));
    assert!(!parsed.breaking_change_body);
}

#[test]
fn test_commit_struct() {
    // Test basic commit parsing
    let commit = Commit::parse("abc123".to_string(), "feat: Add new feature".to_string());
    assert_eq!(commit.commit_id, "abc123");
    assert_eq!(commit.raw_message, "feat: Add new feature");
    assert_eq!(commit.commit_type, "feat");
    assert!(commit.scope.is_none());
    assert!(!commit.breaking_change_flag);
    assert_eq!(commit.title, "Add new feature");
    assert!(commit.body.is_none());
    assert!(!commit.breaking_change_body);
    assert!(commit.author.is_none());
    
    // Test is_major_change
    let commit = Commit::parse("abc123".to_string(), "feat!: Breaking change".to_string());
    assert!(commit.is_major_change());
    
    let commit = Commit::parse("abc123".to_string(), "feat: Add new feature\n\nBREAKING CHANGE: This breaks the old API.".to_string());
    assert!(commit.is_major_change());
    
    // Test is_minor_change
    let commit = Commit::parse("abc123".to_string(), "feat: Add new feature".to_string());
    assert!(commit.is_minor_change());
    
    // Test is_patch_change
    let commit = Commit::parse("abc123".to_string(), "fix: Fix bug".to_string());
    assert!(commit.is_patch_change());
    
    // Test is_noop_change
    let commit = Commit::parse("abc123".to_string(), "chore: Update dependencies".to_string());
    assert!(commit.is_noop_change());
}