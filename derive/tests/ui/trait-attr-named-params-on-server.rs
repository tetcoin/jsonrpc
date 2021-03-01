use tetsy_jsonrpc_derive::rpc;

#[rpc(server, params = "named")]
pub trait Rpc {
}

fn main() {}
