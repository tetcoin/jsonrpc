# Parity JSON-RPC

Rust implementation of JSON-RPC 2.0 Specification.
Transport-agnostic `core` and transport servers for `http`, `ipc`, `websockets` and `tcp`.

**New!** Support for [clients](#Client-support).

[![Build Status][travis-image]][travis-url]
[![Build Status][appveyor-image]][appveyor-url]

[travis-image]: https://travis-ci.org/tetcoin/tetsy-jsonrpc.svg?branch=master
[travis-url]: https://travis-ci.org/tetcoin/tetsy-jsonrpc
[appveyor-image]: https://ci.appveyor.com/api/projects/status/github/tetcoin/tetsy-jsonrpc?svg=true
[appveyor-url]: https://ci.appveyor.com/project/tetcoin/tetsy-jsonrpc/branch/master

[Documentation](http://tetcoin.github.io/tetsy-jsonrpc/)

## Sub-projects
- [tetsy-jsonrpc-core](./core) [![crates.io][core-image]][core-url]
- [tetsy-jsonrpc-core-client](./core-client) [![crates.io][core-client-image]][core-client-url]
- [tetsy-jsonrpc-http-server](./http) [![crates.io][http-server-image]][http-server-url]
- [tetsy-jsonrpc-ipc-server](./ipc) [![crates.io][ipc-server-image]][ipc-server-url]
- [tetsy-jsonrpc-tcp-server](./tcp) [![crates.io][tcp-server-image]][tcp-server-url]
- [tetsy-jsonrpc-ws-server](./ws) [![crates.io][ws-server-image]][ws-server-url]
- [tetsy-jsonrpc-stdio-server](./stdio) [![crates.io][stdio-server-image]][stdio-server-url]
- [tetsy-jsonrpc-derive](./derive) [![crates.io][derive-image]][derive-url]
- [tetsy-jsonrpc-server-utils](./server-utils) [![crates.io][server-utils-image]][server-utils-url]
- [tetsy-jsonrpc-pubsub](./pubsub) [![crates.io][pubsub-image]][pubsub-url]

[core-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-core.svg
[core-url]: https://crates.io/crates/tetsy-jsonrpc-core
[core-client-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-core-client.svg
[core-client-url]: https://crates.io/crates/tetsy-jsonrpc-core-client
[http-server-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-http-server.svg
[http-server-url]: https://crates.io/crates/tetsy-jsonrpc-http-server
[ipc-server-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-ipc-server.svg
[ipc-server-url]: https://crates.io/crates/tetsy-jsonrpc-ipc-server
[tcp-server-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-tcp-server.svg
[tcp-server-url]: https://crates.io/crates/tetsy-jsonrpc-tcp-server
[ws-server-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-ws-server.svg
[ws-server-url]: https://crates.io/crates/tetsy-jsonrpc-ws-server
[stdio-server-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-stdio-server.svg
[stdio-server-url]: https://crates.io/crates/tetsy-jsonrpc-stdio-server
[derive-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-derive.svg
[derive-url]: https://crates.io/crates/tetsy-jsonrpc-derive
[server-utils-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-server-utils.svg
[server-utils-url]: https://crates.io/crates/tetsy-jsonrpc-server-utils
[pubsub-image]: https://img.shields.io/crates/v/tetsy-jsonrpc-pubsub.svg
[pubsub-url]: https://crates.io/crates/tetsy-jsonrpc-pubsub

## Examples

- [core](./core/examples)
- [derive](./derive/examples)
- [pubsub](./pubsub/examples)

### Basic Usage (with HTTP transport)

```rust
use tetsy_jsonrpc_http_server::tetsy_jsonrpc_core::{IoHandler, Value, Params};
use tetsy_jsonrpc_http_server::{ServerBuilder};

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params: Params| {
		Ok(Value::String("hello".to_string()))
	});

	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}
```

### Basic usage with derive

```rust
use tetsy_jsonrpc_core::Result;
use tetsy_jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
	/// Adds two numbers and returns a result
	#[rpc(name = "add")]
	fn add(&self, u64, u64) -> Result<u64>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}
}

fn main() {
	let mut io = tetsy_jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate())
}
```

### Client support

```rust
use tetsy_jsonrpc_core_client::transports::local;
use tetsy_jsonrpc_core::futures::future::{self, Future, FutureResult};
use tetsy_jsonrpc_core::{Error, IoHandler, Result};
use tetsy_jsonrpc_derive::rpc;

/// Rpc trait
#[rpc]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;

	/// Performs asynchronous operation
	#[rpc(name = "callAsync")]
	fn call(&self, a: u64) -> FutureResult<String, Error>;
}

struct RpcImpl;

impl Rpc for RpcImpl {
	fn protocol_version(&self) -> Result<String> {
		Ok("version1".into())
	}

	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}

	fn call(&self, _: u64) -> FutureResult<String, Error> {
		future::ok("OK".to_owned())
	}
}

fn main() {
	let mut io = IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

	let fut = {
		let (client, server) = local::connect::<gen_client::Client, _, _>(io);
		client.add(5, 6).map(|res| println!("5 + 6 = {}", res)).join(server)
	};
	fut.wait().unwrap();
}

```
