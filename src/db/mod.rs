//! Database module for Portfolio v2
//!
//! Provides PostgreSQL connectivity via SQLx with compile-time checked queries.

#[cfg(feature = "ssr")]
mod models;
#[cfg(feature = "ssr")]
mod pool;

#[cfg(feature = "ssr")]
pub use models::*;
#[cfg(feature = "ssr")]
pub use pool::*;
