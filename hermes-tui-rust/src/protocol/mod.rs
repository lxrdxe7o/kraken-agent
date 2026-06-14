//! Protocol module - JSON-RPC protocol implementation
//!
//! This module handles communication with the Hermes gateway via JSON-RPC over stdio.

pub mod types;
pub mod client;
pub mod transport;

pub use types::*;
pub use client::*;
pub use transport::*;
