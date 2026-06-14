//! Hermes TUI Rust - Main Entry Point
//!
//! This is the main binary that launches the Rust TUI for Hermes Agent.
//! It communicates with the Hermes gateway via JSON-RPC over stdio.

use anyhow::{Context, Result};
use hermes_tui_rust::app::App;
use log::{error, info};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Hermes TUI Rust starting up...");

    // Create the app
    let mut app = App::new().context("Failed to initialize application")?;

    // Connect to gateway
    if let Err(e) = app.connect_gateway() {
        error!("Failed to connect to gateway: {}", e);
        // Continue without gateway connection for now
    } else {
        info!("Connected to Hermes gateway");
    }

    if let Err(e) = app.run() {
        error!("Application error: {}", e);
        // cleanup() runs via Drop. Fall through to disconnect_gateway below,
        // then Drop kills the child and restores the terminal.
    }
    // Cleanup
    let _ = app.disconnect_gateway();

    info!("Hermes TUI Rust shutting down gracefully");
    Ok(())
}
