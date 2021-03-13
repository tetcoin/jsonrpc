# tetsy-jsonrpc-tcp-server
TCP server for JSON-RPC 2.0.

[Documentation](http://tetcoin.github.io/tetsy-jsonrpc/tetsy_jsonrpc_tcp_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
tetsy-jsonrpc-tcp-server = "14.2"
```

`main.rs`

```rust
use tetsy_jsonrpc_tcp_server::*;
use tetsy_jsonrpc_tcp_server::tetsy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
```


