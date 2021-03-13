# tetsy-jsonrpc-ipc-server
IPC server (Windows & Linux) for JSON-RPC 2.0.

[Documentation](http://tetcoin.github.io/tetsy-jsonrpc/tetsy_jsonrpc_ipc_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
tetsy-jsonrpc-ipc-server = "14.2"
```

`main.rs`

```rust
extern crate tetsy_jsonrpc_ipc_server;

use tetsy_jsonrpc_ipc_server::ServerBuilder;
use tetsy_jsonrpc_ipc_server::tetsy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".into()))
	});

	let builder = ServerBuilder::new(io);
	let server = builder.start("/tmp/json-ipc-test.ipc").expect("Couldn't open socket");
	server.wait();
}
```

