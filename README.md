# aptu

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
