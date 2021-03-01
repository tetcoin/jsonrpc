//! JSON-RPC client implementation primitives.
//!
//! By default this crate does not implement any transports,
//! use corresponding features (`tls`, `http` or `ws`) to opt-in for them.
//!
//! See documentation of [`tetsy-jsonrpc-client-transports`](../tetsy_jsonrpc_client_transports/) for more details.

#![deny(missing_docs)]

pub use tetsy_jsonrpc_client_transports::*;
