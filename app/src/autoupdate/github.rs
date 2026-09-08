//! Checking GitHub releases for a newer Rook.
//!
//! Upstream's update pipeline serves its own release channels and refuses the
//! `oss` channel outright ("these channels don't ship release artifacts"), which
//! is the one Rook ships on. Rook publishes to GitHub releases instead, so this
//! asks GitHub what the latest tag is.
//!
//! It only reports. Downloading and installing an update is left to the person,
//! because doing it properly means platform-specific install flows and a way to
//! verify what was downloaded, and neither exists here yet.

use std::cmp::Ordering;
use std::time::Duration;

use ::channel_versions::VersionInfo;
use anyhow::{Context, Result};
use serde::Deserialize;

/// Where releases are published. Also what the user is sent to.
pub const RELEASES_URL: &str = "https://github.com/FJRG2007/rook/releases";

const LATEST_RELEASE_API: &str = "https://api.github.com/repos/FJRG2007/rook/releases/latest";

/// Kept short: this runs on a poll, and a hung request must not accumulate.
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// The version a release tag reports as: the tag, verbatim.
///
/// `ChannelState::app_version` hands back `GIT_RELEASE_TAG` exactly as the build
/// passed it - `v0.1.0`, `v` and all - and `should_update` compares the two with
/// `==`. Normalising one side and not the other made every check disagree with a
/// build of that very tag, and the result was a permanent notice offering an
/// update this fork does not install. Ordering the two when they differ is
/// [`compare_release_tags`].
fn version_from_tag(tag: &str) -> VersionInfo {
    VersionInfo::new(tag.to_owned())
}

/// Orders two release tags, or `None` if either is not a version this
/// understands.
///
/// `ParsedVersion` cannot stand in here: its regex requires upstream's
/// `v0.YYYY.MM.DD.HH.MM.channel_NN` shape, so a semver tag fails to parse and
/// `is_current_version_ahead_of_latest_version` returns an error that the caller
/// reads as "not ahead". Every tag that merely differed from the installed one
/// was therefore treated as newer, so a build ahead of the published release -
/// one tagged locally, or one whose release was later yanked - was offered a
/// downgrade on every poll, permanently.
pub fn compare_release_tags(left: &str, right: &str) -> Option<Ordering> {
    Some(ReleaseTag::parse(left)?.cmp(&ReleaseTag::parse(right)?))
}

/// A tag of the shape this fork publishes: `v0.1.0`, optionally with a
/// prerelease suffix and build metadata.
#[derive(Debug, PartialEq, Eq)]
struct ReleaseTag {
    /// The dot-separated numeric components, in order.
    core: Vec<u64>,
    /// The `-rc.1` suffix, split on its dots. Empty for a final release.
    prerelease: Vec<PrereleaseIdentifier>,
}

/// Semver orders a numeric prerelease identifier below an alphanumeric one, and
/// numeric ones among themselves numerically - which the derived ordering gives,
/// in this variant order.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PrereleaseIdentifier {
    Numeric(u64),
    Alphanumeric(String),
}

impl ReleaseTag {
    fn parse(tag: &str) -> Option<Self> {
        // Build metadata is excluded from precedence by the semver spec.
        let tag = tag.strip_prefix('v').unwrap_or(tag);
        let tag = tag.split('+').next().unwrap_or(tag);
        let (core, prerelease) = match tag.split_once('-') {
            Some((core, prerelease)) => (core, Some(prerelease)),
            None => (tag, None),
        };

        let core = core
            .split('.')
            .map(|component| component.parse::<u64>().ok())
            .collect::<Option<Vec<_>>>()?;

        let prerelease = match prerelease {
            Some(prerelease) => prerelease
                .split('.')
                .map(|identifier| {
                    if identifier.is_empty() {
                        None
                    } else if let Ok(number) = identifier.parse::<u64>() {
                        Some(PrereleaseIdentifier::Numeric(number))
                    } else {
                        Some(PrereleaseIdentifier::Alphanumeric(identifier.to_owned()))
                    }
                })
                .collect::<Option<Vec<_>>>()?,
            None => Vec::new(),
        };

        Some(Self { core, prerelease })
    }
}

impl Ord for ReleaseTag {
    fn cmp(&self, other: &Self) -> Ordering {
        // A tag with fewer components is that tag with zeroes appended, so
        // `v0.2` and `v0.2.0` are the same release.
        for index in 0..self.core.len().max(other.core.len()) {
            let ours = self.core.get(index).copied().unwrap_or(0);
            let theirs = other.core.get(index).copied().unwrap_or(0);
            match ours.cmp(&theirs) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }

        // A prerelease comes before the release it leads to.
        match (self.prerelease.is_empty(), other.prerelease.is_empty()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => self.prerelease.cmp(&other.prerelease),
        }
    }
}

impl PartialOrd for ReleaseTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// The subset of GitHub's release payload that matters here.
#[derive(Debug, Deserialize)]
struct LatestRelease {
    tag_name: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
}

/// The version of the most recent published release, as a `VersionInfo` so it
/// travels through the same reporting path as any other update check.
pub async fn fetch_latest_release_version(client: &http_client::Client) -> Result<VersionInfo> {
    let response = client
        .get(LATEST_RELEASE_API)
        // GitHub rejects an API request with no user agent.
        .header("User-Agent", "rook-autoupdate")
        .header("Accept", "application/vnd.github+json")
        .timeout(FETCH_TIMEOUT)
        .send()
        .await
        .context("failed to reach the GitHub releases API")?
        .error_for_status()
        .context("the GitHub releases API returned an error")?;

    let release: LatestRelease = response
        .json()
        .await
        .context("failed to parse the GitHub release payload")?;

    // `/releases/latest` already excludes both, but a future switch to
    // `/releases` would not, and offering someone a draft is worse than
    // offering nothing.
    if release.draft || release.prerelease {
        anyhow::bail!("latest GitHub release is a draft or prerelease");
    }

    Ok(version_from_tag(&release.tag_name))
}

#[cfg(test)]
#[path = "github_tests.rs"]
mod tests;
