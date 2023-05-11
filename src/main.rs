use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

use ldk_node::bitcoin::secp256k1::PublicKey;
use ldk_node::Builder;

fn main() {
	let mut builder = Builder::new();
	builder.set_network("testnet");
	builder.set_storage_dir_path("/tmp/ldk_node_jumpstart".to_string());
	builder.set_esplora_server_url("http://blockstream.info/testnet/api".to_string());

	let node = Arc::new(builder.build());

	node.start().unwrap();

	println!("On-chain funding Address: {}", node.new_funding_address().unwrap());
	println!("Node ID: {}", node.node_id());

	let opennode_testnet_id =
		PublicKey::from_str("02eadbd9e7557375161df8b646776a547c5cbc2e95b3071ec81553f8ec2cea3b8c")
			.unwrap();
	let opennode_testnet_addr = "18.191.253.246:9735".parse().unwrap();

	node.connect(opennode_testnet_id, opennode_testnet_addr, false).unwrap();

	// Handle events by simply printing them as they happen
	let event_node = Arc::clone(&node);
	std::thread::spawn(move || loop {
		let event = event_node.next_event();
		println!("EVENT: {:?}", event);
		event_node.event_handled();
	});

	pause();

	node.disconnect(&opennode_testnet_id).unwrap();

	node.stop().unwrap();
}

fn pause() {
	let mut stdin = io::stdin();
	let mut stdout = io::stdout();

	write!(stdout, "Press any key to continue...").unwrap();
	stdout.flush().unwrap();

	let _ = stdin.read(&mut [0u8]).unwrap();
}
