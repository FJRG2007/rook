# Rook

A terminal for Windows, built from the open-source [Warp](https://github.com/warpdotdev/warp) client and retuned for machines with a discrete GPU.

Upstream is an excellent terminal that behaves poorly on some Windows setups: input and scrolling stutter, opening a tab stalls, and the app slows down the longer it has been installed. Rook keeps the terminal and changes the defaults and storage decisions behind those symptoms.

## What differs from upstream

**Rendering runs on the discrete GPU.** Upstream sets `prefer_low_power_gpu` to true on Windows, so a machine with a dedicated card renders the terminal on the integrated one. Every repaint pays for it: typing, scrolling, selecting text, switching panes. Rook defaults to the discrete GPU on Windows and keeps the integrated one as the crash fallback, so an unstable driver still recovers on its own. Override it with `system.prefer_low_power_gpu` in settings.

**The blocks table is indexed.** `blocks` shipped with no index on `pane_leaf_uuid`, so the retention check that runs after every completed command scanned the whole table, as did session restore at startup. On a 76 MB database that measured 2.14 ms per command before the index and 0.02 ms after.

**Closed panes stop accumulating.** Blocks have no foreign key to their pane, so closing a pane left its captured output in the database permanently. On the machine this was diagnosed on, 1596 of 1640 rows were orphaned and held most of the file. Rook deletes them at startup, when no session is running.

**No update polling.** Upstream polls its own release host every ten minutes. Rook has no such host, so the flag is off and releases are published here instead.

## Install

Download the Windows installer from [Releases](https://github.com/FJRG2007/rook/releases) and run it.

The installer is not code-signed, so SmartScreen shows a warning on first run. Choose "More info" then "Run anyway", or check the file against the checksum on the release page.

## Build from source

Needs Git for Windows (with Git LFS), Visual Studio Build Tools 2022, and Rust, which `rust-toolchain.toml` pins.

```bash
./script/bootstrap    # installs the remaining build dependencies
./script/run          # build and run
./script/presubmit    # fmt, clippy, and tests
```

To produce the installer:

```powershell
.\script\windows\bundle.ps1 -CHANNEL oss -ARCH x64
```

## Tracking upstream

`script/rebrand.py` performs the rename, so upstream changes can be merged by copying the new files in and running it again:

```bash
python script/rebrand.py --dry-run
python script/rebrand.py
```

It preserves what the rename must not touch: third-party repositories under the upstream organization, the five external crates whose package names carry the upstream brand, the protobuf types reached through `api::`, binary assets, and the classifier's tokenizer vocabulary. Renaming any of those breaks the build or the model.

## License and attribution

Rook is a fork of [warpdotdev/warp](https://github.com/warpdotdev/warp) and inherits its licensing. The `rookui_core` and `rookui` crates are [MIT](LICENSE-MIT); everything else is [AGPL v3](LICENSE-AGPL). Warp is a trademark of its owners and this project is not affiliated with or endorsed by them.
