use tetsy_jsonrpc_http_server::tetsy_jsonrpc_core::futures;
use tetsy_jsonrpc_http_server::tetsy_jsonrpc_core::{IoHandler, Value};
use tetsy_jsonrpc_http_server::{hyper, AccessControlAllowOrigin, DomainsValidation, Response, RestApi, ServerBuilder};

fn main() {
	let mut io = IoHandler::default();
	io.add_method("say_hello", |_params| {
		futures::finished(Value::String("hello".to_owned()))
	});

	let server = ServerBuilder::new(io)
		.cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
		.request_middleware(|request: hyper::Request<hyper::Body>| {
			if request.uri() == "/status" {
				Response::ok("Server running OK.").into()
			} else {
				request.into()
			}
		})
		.rest_api(RestApi::Unsecure)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.expect("Unable to start RPC server");

	server.wait();
}
