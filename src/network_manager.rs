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
		let listener = match TcpListener::bind("127.0.0.1:1337") {
			Ok(r) => {
				r
			}
			Err(e) => {
				panic!("Unable to bind TcpListener.");
			}
		};

		for stream in listener.incoming() {
			match stream {
				Ok(result) => {
					println!("Client received!");
				}
				Err(e) => {
					println!("There was an error receiving a client.");
				}
			}
		}
	});

	println!("Started listening...");

	NetworkManager {
		listener_thread
	}
}

impl NetworkManager {
	
}