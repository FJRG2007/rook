# Upstream sync

Rook is a fork of [warpdotdev/Warp](https://github.com/warpdotdev/warp). This file records how far it has been brought forward, so `/catch-up` knows where to start.

**Last synced upstream commit:** `51242b5f0af80fff81613ff6561eed29ba8922fa` (2026-09-07, "[REV-2383] Scope model listing to selected team (#15804)")

## Where Rook deliberately differs

Taking an upstream change on top of one of these would silently undo it. Check before porting anything that touches them.

| Area | Rook's position | Where |
| --- | --- | --- |
| GPU selection on Windows | Defaults to the discrete GPU. Upstream prefers the integrated one, which renders the terminal on the iGPU of any machine with a dedicated card. Crash recovery falls back to integrated rather than out of it. | `app/src/settings/gpu.rs`, `app/src/crash_recovery.rs` |
| `blocks` table | Indexed on `(pane_leaf_uuid, is_background)`, and blocks belonging to closed panes are deleted at startup. Upstream has no index and no cleanup, so the retention count after every command scans the whole table. | `crates/persistence/migrations/2026-09-07-120000_index_blocks_and_command_history`, `app/src/persistence/block_list.rs` |
| Session persistence | Written every 15 seconds and again on termination. Upstream only writes on window move, resize, focus change and close, so a shutdown loses whatever changed since the last of those. | `app/src/persistence/mod.rs`, `app/src/lib.rs` |
| Update checking | Asks the GitHub releases API and reports a newer version; it does not download one. Upstream's version server refuses the `oss` channel outright, so its pipeline could never have served this fork. | `app/src/autoupdate/github.rs`, `app/src/autoupdate/mod.rs` |
| Block retention | A pane drops its oldest blocks past `terminal.max_retained_output_lines`. Upstream evicts nothing, so a pane grows for as long as it is open. | `app/src/terminal/model/blocks.rs`, `app/src/terminal/settings.rs` |
| Finished-block storage | Compacted into flat storage on finish, above a five-row threshold that upstream does not have. Upstream gates the compaction on `MaximizeFlatStorage`, which only their feature-flag server can turn on, so in a fork it never runs at all. | `app/Cargo.toml`, `app/src/features.rs`, `crates/rook_terminal/src/model/grid/grid_handler.rs` |
| CLI-agent plugins | Vendored under `plugins/`, one process per hook. Rook installs these and refuses the upstream builds, which still carry the per-tool-call cost. | `plugins/` |
| Working-directory chip | Clicking it opens the directory in the file manager when it has no menu to show. Upstream leaves that branch of the chip with no click handler at all, which is what it renders while a CLI agent session is running - so for anyone running an agent in the terminal the path is inert. | `app/src/context_chips/display_chip.rs`, `app/src/terminal/view.rs` |
| Tab rename shortcut | F2 renames the focused tab. Upstream ships the action with no default binding. | `app/src/util/bindings.rs` |
| Community link | Discord, not the upstream Slack workspace. | `app/src/util/links.rs` |
| CI and release | Rewritten for this fork: fmt, clippy, tests and a release-profile check on Windows, macOS and Linux, publishing installers to GitHub releases. Upstream's pipelines drive their own infrastructure and cannot run here. Tests run with `--no-fail-fast`, because the default hides every failure behind the first. | `.github/workflows/` |

## What the rename must not touch

`script/rebrand.py` carries the current list and the reasoning. In short: repositories under the upstream organisation other than the client itself, the five external crates whose package names carry the upstream brand, and the protobuf types and fields reached through `api::`. Each of those lives in another repository, so renaming it here only breaks the build.

Those are names the rename must leave alone. There is a second failure mode it cannot prevent, because nothing in the renamed text is wrong: **assertions that depend on a literal it did change.** Four tests broke this way, and all four compiled cleanly.

- `regex_right`, `regex_left` and `nested_regex` search for `Wa.*123` and `Wa.*rp` in fixtures that said `Warp`. The needles are fragments, not the whole word, so they stayed put while the haystack became `Rook` and the searches began returning `None`.
- `test_find_url_omits_trailing_periods` hard-codes the columns a URL occupies. `github.com/warpdotdev/Warp` became `github.com/FJRG2007/rook` and lost two characters; the column numbers did not move.
- `deserialize_mixed_environment_uses_per_repo_forges` is the inverse: its fixture's `"repo": "warp"` was renamed to `"rook"`, while the clone URL it asserts, `https://github.com/warpdotdev/warp.git`, was *deliberately preserved* by the rule that protects the upstream repository. The protection and the rename disagreed, and the test compared one against the other.

After re-running the rename, run the tests on every platform rather than only building. A fragment of the brand used as a search pattern, and a hard-coded length or offset into a renamed literal, are the two shapes to look for.

## Log

### 2026-09-07 - initial import

Imported at `51242b5`. No upstream changes ported yet; everything since is what `/catch-up` will report first.
