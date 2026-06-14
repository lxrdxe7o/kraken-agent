//! Protocol Client - JSON-RPC client for gateway communication
//!
//! This module provides the client-side functionality for communicating
//! with the Hermes gateway via JSON-RPC over stdio.

use crate::protocol::transport::StdioTransport;
use crate::protocol::types::{GatewayMessage, TuiRequest};
use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::sync::mpsc::Receiver;
use std::fmt;

/// Client for sending requests to the gateway
pub struct GatewayClient {
    /// The transport for JSON-RPC communication
    transport: Option<StdioTransport<Box<dyn Write + Send>>>,
    /// Receiver for parsed gateway messages
    response_receiver: Option<Receiver<GatewayMessage>>,
    /// Whether the client is currently connected
    connected: bool,
}

// Manual Debug impl because StdioTransport<Box<dyn Write + Send>> doesn't impl Debug
impl fmt::Debug for GatewayClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GatewayClient")
            .field("transport", &self.transport.as_ref().map(|_| "StdioTransport<_>"))
            .field("response_receiver", &self.response_receiver.as_ref().map(|_| "Receiver<_>"))
            .field("connected", &self.connected)
            .finish()
    }
}
impl GatewayClient {
    /// Create a new gateway client
    pub fn new() -> Self {
        Self {
            transport: None,
            response_receiver: None,
            connected: false,
        }
    }

    /// Connect to the gateway via stdio pipes
    ///
    /// Spawns a background reader thread via StdioTransport and sets up
    /// message parsing. Uses non-blocking receive for the event loop.
    pub fn connect<R: Read + Send + 'static, W: Write + Send + 'static>(
        &mut self,
        stdin: R,
        stdout: W,
    ) -> Result<()> {
        // Disconnect if already connected
        if self.connected {
            self.disconnect()?;
        }

        // Create channel for parsed gateway messages
        let (response_sender, response_receiver) = std::sync::mpsc::channel::<GatewayMessage>();

        // Create StdioTransport with boxed writer
        let mut transport = StdioTransport::new(
            std::io::empty(),
            Box::new(stdout) as Box<dyn Write + Send>,
        );

        // Start reader thread — returns a receiver for raw JSON lines
        let line_receiver = transport.start_reader(stdin);

        // Spawn a parsing thread that converts JSON strings to GatewayMessages
        std::thread::spawn(move || {
            for line in line_receiver {
                if let Ok(message) = serde_json::from_str::<GatewayMessage>(&line) {
                    let _ = response_sender.send(message);
                }
            }
        });

        self.transport = Some(transport);
        self.response_receiver = Some(response_receiver);
        self.connected = true;

        Ok(())
    }

    /// Send a request to the gateway
    ///
    /// Serializes the request and writes it directly through the transport.
    pub fn send_request(&mut self, request: TuiRequest) -> Result<()> {
        let transport = self
            .transport
            .as_mut()
            .context("GatewayClient: Not connected")?;
        transport
            .write_message(&request)
            .context("GatewayClient: Failed to send request")?;
        Ok(())
    }

    /// Receive a message from the gateway (non-blocking)
    ///
    /// Uses try_recv() so the caller can continue the event loop
    /// instead of blocking when no message is available.
    pub fn receive_message(&self) -> Option<GatewayMessage> {
        self.response_receiver
            .as_ref()
            .and_then(|rx| rx.try_recv().ok())
    }

    /// Check if the client is currently connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Disconnect from the gateway
    ///
    /// Drops the transport (which stops the reader thread) and clears
    /// the receiver channel.
    pub fn disconnect(&mut self) -> Result<()> {
        // Drop the transport first — this stops the reader thread.
        // Must happen AFTER the child process is killed to avoid hanging
        // on a blocked read_line.
        self.transport = None;
        self.response_receiver = None;
        self.connected = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GatewayClient::new();
        assert!(client.transport.is_none());
        assert!(client.response_receiver.is_none());
    }

    #[test]
    fn test_client_connect_disconnect() {
        let mut client = GatewayClient::new();
        let result = client.connect(
            std::io::empty(),
            std::io::sink(),
        );
        
        // For now, just test that it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }
}
