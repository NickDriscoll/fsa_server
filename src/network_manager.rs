use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::thread;
use std::panic;
use std::vec::Vec;
use std::sync::mpsc;
use std::io::Read;

const MAX_PLAYERS: usize = 4;
const BUFFER_SIZE: usize = 1;

pub struct NetworkManager {
	listener_thread: thread::JoinHandle<()>,
	phones: Vec<TcpStream>,
	remove_indices: Vec<usize>,
	rx: mpsc::Receiver<TcpStream>
}

//Start a thread to listen for incoming client connections
pub fn begin_listening() -> NetworkManager {
	let (tx, rx) = mpsc::channel();

	let listener_thread = thread::spawn(move || {
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
			println!("Stream detected.");
			let stream = stream.unwrap();
			stream.set_nonblocking(true);
			match tx.send(stream) {
				Err(e) => {
					println!("An error occurred while sending data over a channel: {:?}", e);
				}
				_ => { }
			}
		}
	});

	println!("Started listening...");

	NetworkManager {
		listener_thread,
		phones: Vec::with_capacity(MAX_PLAYERS),
		remove_indices: Vec::with_capacity(MAX_PLAYERS),
		rx
	}
}

impl NetworkManager {
	pub fn handle_input(&mut self) {
		//First thing to do is check if there are any pending connections
		match self.rx.try_recv() {
			Ok(stream) => {
				if (self.phones.len() < MAX_PLAYERS) {
					self.phones.push(stream);
				}
			}
			_ => { }
		}

		//For each stream, emit commands based on what's in the stream buffer
		let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
		for (i, stream) in self.phones.iter_mut().enumerate() {
			match stream.read(&mut buffer) {
				Ok(0) => {					
					println!("Phone disconnected.");
					self.remove_indices.push(i);
				}
				Ok(n) => {
					//Command emission actually happens in this case
					if buffer[0] != 0 {
						println!("Received {:#x}", buffer[0]);
					}
				}
				Err(e) => { }
			}
		}

		//Remove disconnected phones
		for i in self.remove_indices.iter() {
			self.phones.remove(*i);
		}
		self.remove_indices.clear();
	}
}