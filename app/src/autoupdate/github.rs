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
/// build of that very tag, and the mismatch does not stop there: the fallback
/// comparison parses both through `ParsedVersion`, whose regex requires
/// upstream's `v0.YYYY.MM.DD.HH.MM.channel_NN` shape, so a semver tag fails to
/// parse and the check falls through to reporting an update. The result was a
/// permanent notice offering an update this fork does not install.
fn version_from_tag(tag: &str) -> VersionInfo {
    VersionInfo::new(tag.to_owned())
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
