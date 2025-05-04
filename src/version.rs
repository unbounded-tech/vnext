use semver::{BuildMetadata, Prerelease, Version};

pub struct VersionBump {
    pub major: bool,
    pub minor: bool,
    pub patch: bool,
}

pub struct CommitSummary {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub noop: u32,
}

pub fn parse_version(tag: &str) -> Result<Version, semver::Error> {
    let cleaned_tag = tag.trim_start_matches('v');
    Version::parse(cleaned_tag)
}

pub fn calculate_next_version(current: &Version, bump: &VersionBump) -> Version {
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

        let bump = VersionBump {
            major: false,
            minor: false,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 3));

        let bump = VersionBump {
            major: false,
            minor: false,
            patch: true,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 2, 4));

        let bump = VersionBump {
            major: false,
            minor: true,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(1, 3, 0));

        let bump = VersionBump {
            major: true,
            minor: false,
            patch: false,
        };
        assert_eq!(calculate_next_version(&base, &bump), Version::new(2, 0, 0));
    }
}