use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::thread;
use std::panic;
use std::vec::Vec;
use std::sync::mpsc;
use std::io::Read;
use command::Command;

const MAX_PLAYERS: usize = 4;
const BUFFER_SIZE: usize = 1;

pub struct NetworkManager<'a> {
	listener_thread: thread::JoinHandle<()>,
	phones: Vec<TcpStream>,
	remove_indices: Vec<usize>,
	rx: mpsc::Receiver<TcpStream>,
	bitmask_map: HashMap<u8, Command<'a>>
}

//Start a thread to listen for incoming client connections
pub fn begin_listening<'a>() -> NetworkManager<'a> {
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
		rx,
		bitmask_map: HashMap::new()
	}
}

impl<'a> NetworkManager<'a> {
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
					for i in 1..8 {
						if (i & buffer[0]) != 0 {
							match self.bitmask_map.get_mut(&(i & buffer[0])) {
								Some(command) => {
									command.execute();
								}
								None => {

								}
							}
						}
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

	pub fn add_touchdown_binding(&mut self, mask: u8, command: Command<'a>) {
		self.bitmask_map.insert(mask, command);
	}
}