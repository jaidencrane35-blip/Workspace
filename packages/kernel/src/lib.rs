//! Workspace Platform Kernel — future core runtime boundary.
//!
//! Sprint 01: placeholder crate only.
//! Do not add automation, AI, Windows APIs, or state management here yet.

#![deny(clippy::all)]

/// Kernel crate version aligned with application semver.
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Placeholder module for future platform kernel services.
pub mod core {
    //! Future home of event bus, permission gateway, and configuration.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_version_is_set() {
        assert_eq!(KERNEL_VERSION, "0.1.0");
    }
}
