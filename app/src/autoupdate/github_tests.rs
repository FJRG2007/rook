use super::*;

/// The regression this file exists for. `ChannelState::app_version` returns
/// `GIT_RELEASE_TAG` verbatim, and `AutoupdateState::should_update` decides
/// "already up to date" by comparing that string to this one with `==`. Trimming
/// the `v` here made the two disagree for a build of the very tag being offered,
/// and every check reported an update the fork has no way to install.
#[test]
fn a_release_reports_the_tag_verbatim() {
    assert_eq!(version_from_tag("v0.1.0").version, "v0.1.0");
}

/// Nothing is stripped, trimmed or reordered on the way through, whatever the
/// tag looks like - including the shape upstream's version server uses, which
/// is the only one `ParsedVersion` can parse.
#[test]
fn no_part_of_a_tag_is_rewritten() {
    for tag in [
        "v0.1.0",
        "v1.2.3-rc.1",
        "v0.2023.05.15.08.04.stable_01",
        "0.1.0",
    ] {
        assert_eq!(version_from_tag(tag).version, tag);
    }
}
