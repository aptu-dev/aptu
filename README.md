<p align="center">
  <a href="https://github.com/aptu-dev/aptu/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/badge/CI-passing-brightgreen.svg?style=for-the-badge" height="20"></a>
  <a href="https://github.com/aptu-dev/aptu/actions/workflows/scorecard.yml"><img alt="Scorecard" src="https://img.shields.io/badge/Scorecard-enabled-blue.svg?style=for-the-badge" height="20"></a>
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge" height="20"></a>
</p>

<h1 align="center">aptu</h1>

Placeholder crate reserving the `aptu` name for [aptu.dev](https://aptu.dev):
machine-payable API infrastructure for autonomous agents (x402, HTTP 402,
USDC on Base).

The production x402 middleware (replay guard, facilitator failover, discovery
manifests) currently runs on Cloudflare Workers in TypeScript. Any Rust
implementation will land here; until then this crate is intentionally minimal.

Related:

- Field notes: <https://clouatre.ca/posts/agent-payments-x402/>
- x402 protocol: <https://www.x402.org/>

## License

MIT OR Apache-2.0
