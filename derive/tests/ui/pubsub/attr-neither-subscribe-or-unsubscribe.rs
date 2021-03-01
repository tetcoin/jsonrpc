#[macro_use]
extern crate tetsy_jsonrpc_derive;
extern crate tetsy_jsonrpc_core;
extern crate tetsy_jsonrpc_pubsub;

#[rpc]
pub trait Rpc {
	type Metadata;

	/// Hello subscription
	#[pubsub(subscription = "hello", name = "hello_subscribe", alias("hello_sub"))]
	fn subscribe(&self, _: Self::Metadata, _: typed::Subscriber<String>, _: u64);

	/// Unsubscribe from hello subscription.
	#[pubsub(subscription = "hello", unsubscribe, name = "hello_unsubscribe")]
	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}

fn main() {}
