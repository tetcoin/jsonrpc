use env_logger;
use tetsy_jsonrpc_tcp_server::tetsy_jsonrpc_core::*;
use tetsy_jsonrpc_tcp_server::ServerBuilder;

fn main() {
	env_logger::init();
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		println!("Processing");
		Ok(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait()
}
