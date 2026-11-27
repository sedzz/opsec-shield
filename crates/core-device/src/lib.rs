//! Device visibility core.
//!
//! A pure library with no UI or network dependencies, so it can be reused by
//! the CLI today and by other front-ends or a server later.

/// Returns the core version, useful to check that the CLI links correctly.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }
}
