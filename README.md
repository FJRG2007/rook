<a href="https://www.rook.dev">
    <img width="1024" alt="Rook Agentic Development Environment product preview" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>
&nbsp;
<p align="center">
  <a href="https://rook.dev/factories"><img height="20" alt="Built with Rook" src="https://raw.githubusercontent.com/warpdotdev/brand-assets/main/Github/Built-With-Rook-Export@2x.png" /></a>
</p>

<p align="center">
  <a href="https://www.rook.dev">Website</a>
  ·
  <a href="https://www.rook.dev/code">Code</a>
  ·
  <a href="https://www.rook.dev/agents">Agents</a>
  ·
  <a href="https://www.rook.dev/terminal">Terminal</a>
  ·
  <a href="https://www.rook.dev/drive">Drive</a>
  ·
  <a href="https://docs.rook.dev">Docs</a>
  ·
  <a href="https://www.rook.dev/blog/how-rook-works">How Rook Works</a>
</p>

> [!NOTE]
> OpenAI is the founding sponsor of the new, open-source Rook repository, and the new agentic management workflows are powered by GPT models.

<h1></h1>

## About

[Rook](https://www.rook.dev) is an agentic development environment, born out of the terminal. Use Rook's built-in coding agent, or bring your own CLI agent (Claude Code, Codex, Gemini CLI, and others).

## Installation

You can [download Rook](https://www.rook.dev/download) and [read our docs](https://docs.rook.dev/) for platform-specific instructions.

## Rook Contributions Overview Dashboard

Explore [build.rook.dev](https://build.rook.dev) to:
- Watch thousands of [Rook Factory](rook.dev/factories) agents triage issues, write specs, implement changes, and review PRs
- View top contributors and in-flight features
- Track your own issues with GitHub sign-in
- Click into active agent sessions in a web-compiled Rook terminal

## Automate development with Rook Factories

This repository is driven by [Rook Factories](https://rook.dev/factories): open, flexible infrastructure for teams to build cloud software factories of their own.

Rook Factories are defined in code and easy to deploy on any model or harness, with evals, benchmarks, and self-improvement built in. [Request early access](rook.dev/factories/request-access).

## Licensing

Rook's UI framework (the `rookui_core` and `rookui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source & Contributing

Rook's client codebase is open source and lives in this repository. We welcome community contributions and have designed a lightweight workflow to help new contributors get started. For the full contribution flow, read our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

> [!TIP]
> **Chat with contributors and the Rook team** in the [`#oss-contributors`](https://rookcommunity.slack.com/archives/C0B0LM8N4DB) Slack channel — a good place for ad-hoc questions, design discussion, and pairing with maintainers. New here? [Join the Rook Slack community](https://go.rook.dev/join-preview) first, then jump into `#oss-contributors`.

### Issue to PR

Before filing, [search existing issues](https://github.com/FJRG2007/rook/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc) for your bug or feature request. If nothing exists, [file an issue](https://github.com/FJRG2007/rook/issues/new/choose) using our templates. Security vulnerabilities should be reported privately as described in [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues).

Once filed, a Rook maintainer reviews the issue and may apply a readiness label: [`ready-to-spec`](https://github.com/FJRG2007/rook/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec) signals the design is open for contributors to spec out, and [`ready-to-implement`](https://github.com/FJRG2007/rook/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement) signals the design is settled and code PRs are welcome. Anyone can pick up a labeled issue — mention **@oss-maintainers** on an issue if you'd like it considered for a readiness label.

### Building the Repo Locally

To build and run Rook from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Rook
./script/presubmit   # fmt, clippy, and tests
```

See [AGENTS.md](AGENTS.md) for the full engineering guide, including coding style, testing, and platform-specific notes.

## Joining the Team

Interested in joining the team? See our [open roles](https://www.rook.dev/careers).

## Support and Questions

1. See our [docs](https://docs.rook.dev/) for a comprehensive guide to Rook's features.
2. Join our [Slack Community](https://go.rook.dev/join-preview) to connect with other users and get help from the Rook team — contributors hang out in [`#oss-contributors`](https://rookcommunity.slack.com/archives/C0B0LM8N4DB).
3. Try our [Preview build](https://www.rook.dev/download-preview) to test the latest experimental features.
4. Mention **@oss-maintainers** on any issue to escalate to the team — for example, if you encounter problems with the automated agents.

## Code of Conduct

We ask everyone to be respectful and empathetic. Rook follows the [Code of Conduct](CODE_OF_CONDUCT.md). To report violations, email rook-coc at rook.dev.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.rook.dev/help/licenses) that have helped Rook to get off the ground:

- [Tokio](https://github.com/tokio-rs/tokio)
- [NuShell](https://github.com/nushell/nushell)
- [Fig Completion Specs](https://github.com/withfig/autocomplete)
- [Rook Server Framework](https://github.com/seanmonstar/rook)
- [Alacritty](https://github.com/alacritty/alacritty)
- [Hyper HTTP library](https://github.com/hyperium/hyper)
- [FontKit](https://github.com/servo/font-kit)
- [Core-foundation](https://github.com/servo/core-foundation-rs)
- [Smol](https://github.com/smol-rs/smol)
