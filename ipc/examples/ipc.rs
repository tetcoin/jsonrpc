use tetsy_jsonrpc_ipc_server;

use tetsy_jsonrpc_ipc_server::tetsy_jsonrpc_core::*;

fn main() {
	let mut io = MetaIoHandler::<()>::default();
	io.add_method("say_hello", |_params| Ok(Value::String("hello".to_string())));
	let _server = tetsy_jsonrpc_ipc_server::ServerBuilder::new(io)
		.start("/tmp/tetsy-example.ipc")
		.expect("Server should start ok");
}
