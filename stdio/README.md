# tetsy-jsonrpc-stdio-server
STDIN/STDOUT server for JSON-RPC 2.0.
Takes one request per line and outputs each response on a new line.

[Documentation](http://tetcoin.github.io/tetsy-jsonrpc/tetsy_jsonrpc_stdio_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
tetsy-jsonrpc-stdio-server = "14.2"
```

`main.rs`

```rust
use tetsy_jsonrpc_stdio_server::ServerBuilder;
use tetsy_jsonrpc_stdio_server::tetsy_jsonrpc_core::*;

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".to_owned()))
	});

	ServerBuilder::new(io).build();
}
```
