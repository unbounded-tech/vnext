//! Version-related data structures

/// Represents the type of version bump to apply
pub struct VersionBump {
    pub major: bool,
    pub minor: bool,
    pub patch: bool,
}