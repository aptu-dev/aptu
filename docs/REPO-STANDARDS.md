# Repository Standards

Per-repo governance layer for `aptu-dev/aptu`. The org-level baseline is
defined in [ORG-STANDARDS.md](https://github.com/aptu-dev/.github/blob/main/ORG-STANDARDS.md)
and enforced by the org rulesets managed from `aptu-dev/.github`; this
document maps the artifacts that implement the baseline here and records
the (few) overrides.

## Org ruleset compliance

- **Main Branch Protection** (org ruleset, `evaluate` during rollout):
  applies to `main`; requires pull-request-only merging, signed commits,
  and the `DCO` status check. No per-repo divergence.
- **Release Tag Protection** and **Tag Immutability** (org rulesets,
  `active`): cover all `v*` tags. No per-repo divergence.
- **Required checks**: the org ruleset requires `DCO` only. This repo
  additionally treats `CI Result` as its aggregate check once the org
  ruleset is promoted to `active`; per-repo rulesets will be added at
  that point if repo-specific checks are needed.

## Artifact map

| Path | Purpose |
|------|---------|
| `AGENTS.md` | AI agent project context |
| `LICENSE`, `LICENSES/` | Dual license (MIT OR Apache-2.0) texts |
| `REUSE.toml` | SPDX copyright and license annotations for all repo files |
| `.commitlintrc.yml` | Conventional Commits ruleset for commitlint |
| `.markdownlint.jsonc` | markdownlint-cli2 configuration |
| `.github/workflows/ci.yml` | Commitlint, secret scan, cargo build + test; aggregates into `CI Result` |
| `.github/workflows/markdown-lint.yml` | markdownlint-cli2 on PRs to `main` |
| `.github/workflows/reuse.yml` | FSFE reuse-action compliance check |
| `.github/workflows/scorecard.yml` | Weekly OpenSSF Scorecard (public repo) |
| `.github/scorecard-policy.yml` | Scorecard SARIF policy |
| `.github/workflows/aptu-review.yml` | AI-assisted PR review via the aptu App |
| `.github/workflows/aptu-scan-security.yml` | Automated security scan via the aptu App |
| `.github/workflows/aptu-triage.yml` | Automatic issue triage via the aptu App |

## Overrides

None. No repo-level rulesets, divergent merge settings, or extra
required checks beyond the org baseline at this time; revisit when the
crate grows beyond the placeholder.
