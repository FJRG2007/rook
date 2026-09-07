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

The script's header explains how the list is maintained: by compiling. Each missing exception shows up as an unknown field or variant on a type this repository does not own.

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
.\script\windows\bundle.ps1 -CHANNEL oss -ARCH x64    # the installer
```

CI runs fmt, clippy, tests and a release-profile check on Windows. Pushing a `v*` tag builds the installer and attaches it to the GitHub release. Installers are unsigned; there is no certificate.

## Performance work

The fork exists because upstream is slow on this hardware, so performance changes want evidence rather than reasoning. What has been measured so far, and how, is in `docs/upstream-sync.md` and in the commit messages of the `perf` and `fix` commits. When adding to it: measure on a real database or a real log before and after, and put the numbers in the commit message.
