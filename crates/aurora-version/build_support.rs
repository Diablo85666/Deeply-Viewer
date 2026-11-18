//! Pure version resolution, shared by the build script and integration tests.

use semver::{BuildMetadata, Version};

#[derive(Debug, PartialEq, Eq)]
pub struct BuildVersion {
    pub display: String,
    pub login: String,
}

fn release_version(tag: &str) -> Result<Version, String> {
    let number = tag.strip_prefix('v').ok_or_else(|| "a release tag must start with v".to_owned())?;
    if let Some((base, build)) = number.rsplit_once('.')
        && base.split('.').count() == 3
        && base
            .split('.')
            .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()))
        && build.chars().all(|c| c.is_ascii_digit())
    {
        return Version::parse(&format!("{base}+{build}")).map_err(|error| format!("invalid release tag {tag:?}: {error}"));
    }
    Version::parse(number).map_err(|error| format!("invalid release tag {tag:?}: {error}"))
}

fn describe_version(description: &str) -> Result<(Version, u64, &str), String> {
    // Split from the right: prerelease tags themselves can contain dashes.
    let mut parts = description.rsplitn(3, '-');
    let hash = parts.next().and_then(|part| part.strip_prefix('g'));
    let distance = parts.next().and_then(|part| part.parse::<u64>().ok());
    let tag = parts.next();
    match (tag, distance, hash) {
        (Some(tag), Some(distance), Some(hash)) if !hash.is_empty() && hash.bytes().all(|c| c.is_ascii_hexdigit()) => {
            Ok((release_version(tag)?, distance, hash))
        }
        _ => Err(format!("invalid git describe output {description:?}")),
    }
}

pub fn resolve(
    release_tag: Option<&str>,
    description: Option<&str>,
    commit_count: u64,
    commit: Option<&str>,
) -> Result<BuildVersion, String> {
    let (mut version, distance, hash) = if let Some(tag) = release_tag {
        let version = release_version(tag)?;
        if let Some(description) = description {
            let (git_version, distance, _) = describe_version(description)?;
            if git_version != version || distance != 0 {
                return Err(format!(
                    "release tag {tag:?} does not describe the checked-out commit ({description})"
                ));
            }
        }
        (version, 0, "")
    } else if let Some(description) = description {
        describe_version(description)?
    } else {
        // A source archive without Git must never pretend to be Cargo's old
        // package version. Official archives can supply DEEPLY_VERSION_TAG.
        (Version::new(0, 0, 0), commit_count, commit.unwrap_or("unknown"))
    };
    let custom_release = release_tag.map(|tag| tag.strip_prefix('v').unwrap_or(tag)).filter(|tag| {
        let mut parts = tag.split('.');
        matches!(
            (parts.next(), parts.next(), parts.next(), parts.next(), parts.next()),
            (Some(a), Some(b), Some(c), Some(d), None)
                if [a, b, c, d].iter().all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
        )
    });
    let custom_display = custom_release.map(str::to_owned);
    let login = custom_display
        .clone()
        .unwrap_or_else(|| format!("{}.{}.{}.{}", version.major, version.minor, version.patch, distance));
    if distance > 0 || (release_tag.is_none() && description.is_none()) {
        let provenance = format!("{distance}.g{hash}");
        let metadata = if version.build.is_empty() {
            provenance
        } else {
            format!("{}.{provenance}", version.build)
        };
        version.build = BuildMetadata::new(&metadata).map_err(|error| format!("invalid build provenance: {error}"))?;
    }
    Ok(BuildVersion {
        display: custom_display.unwrap_or_else(|| version.to_string()),
        login,
    })
}
