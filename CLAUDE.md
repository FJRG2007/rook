# Rook

A fork of [warpdotdev/Warp](https://github.com/warpdotdev/warp), rebranded and retuned for Windows. `AGENTS.md` is the inherited engineering guide and still applies; this file covers what is different because Rook is a fork.

## Staying current with upstream

Run `/catch-up` to pull in what upstream has changed, port what applies, and summarise the rest. Worth doing at the start of a session that has not run it in a while, and any time the question "has upstream fixed this already?" comes up.

`docs/upstream-sync.md` records how far the fork has been brought forward and, more importantly, **where Rook deliberately differs from upstream**. Read that table before porting anything: taking an upstream change on top of one of those areas silently undoes a fix.

## The rename

`script/rebrand.py` turns the upstream tree into this one (`WARP` -> `ROOK`, `Warp` -> `Rook`, `warp` -> `rook`, in file contents and paths). Re-run it after copying upstream files in.

It deliberately leaves some names alone, and every exception exists because renaming it broke the build:

- Repositories under the upstream organisation other than the client itself. They are real third-party forks this build depends on (`rust-objc`, `winit`, `vte`, `warp-proto-apis`, the common-skills scripts).
- The five external crates whose package names carry the upstream brand, listed in the script. Their names live in another repository.
- Protobuf types and fields reached through the `api::` alias, plus the handful reached without it. The wire format owns those names.
- Binary assets, and the input classifier's tokenizer vocabulary.
- `warp pointer` in `crates/computer_use/src/linux/x11/`, where it is the English word: X11's `XWarpPointer` teleports the cursor. Local code, not a foreign name, so it is the one exception the reasoning above does not cover.

The script's header explains how the list is maintained: by compiling. Each missing exception shows up as an unknown field or variant on a type this repository does not own.

## Importing files from a checkout

Copy with something that preserves the executable bit. The original import used
`robocopy`, which does not, so all 60 executable files under `script/` landed as
644 and every macOS and Linux CI job died in setup with exit code 126 - "found,
but not executable". Nothing about that failure names the mode bit.

After copying, restore it from the source checkout rather than guessing:

```bash
git -C <source> ls-files -s | awk '$1=="100755"{print $2}'   # then update-index --chmod=+x
```

Windows has `core.filemode=false`, so git will not notice the difference on its
own and the working tree looks fine locally.

## Verifying a change

```bash
cargo check -p rook                        # the app; does NOT build test targets
cargo check --workspace --all-targets \
  --exclude rook_js --exclude command-signatures-v2
```

The second one is what catches errors in tests, and it needs libclang for `bindgen`. If libclang is not installed locally, say that rather than reporting that the tests compile - CI covers it. `rook_js` targets wasm and `command-signatures-v2` needs its JS toolchain built first, so neither builds on Windows.

## Building and releasing

```bash
./script/run                                          # build and run
.\script\windows\bundle.ps1 -CHANNEL oss -ARCH x64    # the Windows installer
./script/macos/bundle --channel oss --nosign          # the macOS disk image
./script/linux/bundle --channel oss --packages appimage
```

CI runs clippy, tests and a release-profile check on Windows, macOS and Linux, and `cargo fmt --check` on Linux alone, since formatting is platform independent. Pushing a `v*` tag builds all three installers and attaches them to the GitHub release. All three are unsigned; this repository holds neither an Apple certificate nor a Windows one.

## Performance work

The fork exists because upstream is slow on this hardware, so performance changes want evidence rather than reasoning. What has been measured so far, and how, is in `docs/upstream-sync.md` and in the commit messages of the `perf` and `fix` commits. When adding to it: measure on a real database or a real log before and after, and put the numbers in the commit message.
