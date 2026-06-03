mod client;
mod error;

pub mod auth;
pub mod sessions;
pub mod shared_memory;
pub mod spaces;

pub use client::{DEFAULT_BASE_URL, KitKitClient, KitKitClientConfig};
pub use error::{KitKitError, Result};
