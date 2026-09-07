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
| Community link | Discord, not the upstream Slack workspace. | `app/src/util/links.rs` |
| CI and release | Rewritten for this fork: Windows build, test and release to GitHub releases. Upstream's pipelines drive their own infrastructure and cannot run here. | `.github/workflows/` |

## What the rename must not touch

`script/rebrand.py` carries the current list and the reasoning. In short: repositories under the upstream organisation other than the client itself, the five external crates whose package names carry the upstream brand, and the protobuf types and fields reached through `api::`. Each of those lives in another repository, so renaming it here only breaks the build.

## Log

### 2026-09-07 - initial import

Imported at `51242b5`. No upstream changes ported yet; everything since is what `/catch-up` will report first.
