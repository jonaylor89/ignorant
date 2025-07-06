//! Platform checking modules
//! 
//! Each module implements a specific platform checker for determining
//! if a phone number is associated with an account on that platform.

/// Amazon account checking via login form analysis
pub mod amazon;

/// Instagram account checking via internal API
pub mod instagram;

/// Snapchat account checking via registration validation
pub mod snapchat;