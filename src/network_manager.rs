use std::net::TcpListener;
use std::collections::HashMap;
use std::thread;
use std::panic;

pub struct NetworkManager {
	listener_thread: thread::JoinHandle<()>
}

//Start a thread to listen for incoming client connections
pub fn begin_listening() -> NetworkManager {
	let listener_thread = thread::spawn(|| {
		let listener = match TcpListener::bind("0.0.0.0:1337") {
			Ok(r) => {
				println!("Bound to port 1337.");
				r
			}
			Err(e) => {
				panic!("Unable to bind TcpListener.");
			}
		};
		println!("Listener is {:?}", listener);

		for stream in listener.incoming() {
			println!("Stream detected.")
		}
	});

	println!("Started listening...");

	NetworkManager {
		listener_thread
	}
}

impl NetworkManager {
	
}