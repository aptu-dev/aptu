//! Placeholder crate reserving the `aptu` name for <https://aptu.dev>.
//!
//! Machine-payable API infrastructure for autonomous agents (x402).
//! Intentionally minimal; see the README for status.

/// Crate version marker; the real API will replace this.
pub const PLACEHOLDER: &str = "aptu 0.0.1 (placeholder for aptu.dev)";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder_is_present() {
        assert!(PLACEHOLDER.contains("aptu"));
    }
}
