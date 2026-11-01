# AGENTS.md

Project context for AI coding agents working on `aptu-dev/aptu`.

## Overview

Placeholder crate reserving the `aptu` name for aptu.dev: machine-payable
API infrastructure for autonomous agents (x402). Intentionally minimal;
see the README for status and `docs/REPO-STANDARDS.md` for the governance
baseline.

## Commands

- `cargo build` - build
- `cargo test` - test
- `cargo clippy --profile ci --locked` - lint (CI parity)

## Conventions

- Conventional Commits enforced (commitlint in CI); every commit needs a
  DCO sign-off and GPG/SSH signature (`git commit -S --signoff`).
- Squash-merge only (org setting); branch names feature-scoped.
- REUSE licensing: run `reuse lint` locally after adding files; keep
  `REUSE.toml` annotations current.
- ASCII-only in all committed files.
