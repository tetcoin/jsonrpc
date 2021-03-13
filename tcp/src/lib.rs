//! jsonrpc server over tcp/ip
//!
//! ```no_run
//! use tetsy_jsonrpc_core::*;
//! use tetsy_jsonrpc_tcp_server::ServerBuilder;
//!
//! fn main() {
//! 	let mut io = IoHandler::default();
//! 	io.add_method("say_hello", |_params| {
//! 		Ok(Value::String("hello".to_string()))
//! 	});
//! 	let server = ServerBuilder::new(io)
//!			.start(&"0.0.0.0:0".parse().unwrap())
//!			.expect("Server must start with no issues.");
//!
//!		server.wait();
//! }
//! ```

#![deny(missing_docs)]

use tetsy_jsonrpc_server_utils as server_utils;

pub use tetsy_jsonrpc_core;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod dispatch;
mod meta;
mod server;
mod service;

#[cfg(test)]
mod logger;
#[cfg(test)]
mod tests;

use tetsy_jsonrpc_core as jsonrpc;

pub use self::server_utils::{codecs::Separator, tokio};
pub use crate::dispatch::{Dispatcher, PushMessageError};
pub use crate::meta::{MetaExtractor, RequestContext};
pub use crate::server::{Server, ServerBuilder};
