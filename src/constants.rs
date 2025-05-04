// Constant regex string literals used for defaults.
pub const MAJOR_REGEX_STR: &str = r"(?m)^major(\(.+\))?:.*";
pub const MINOR_REGEX_STR: &str = r"(?m)^(minor|feat)(\(.+\))?:.*";
pub const NOOP_REGEX_STR: &str = r"(?m)^(noop|chore)(\(.+\))?:.*";
pub const BREAKING_REGEX_STR: &str = r"(?m)^BREAKING CHANGE:.*";