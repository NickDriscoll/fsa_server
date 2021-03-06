use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;
use std::thread;
use std::vec::Vec;
use std::sync::mpsc;
use std::io::Read;
use command::Command;
use command::CommandEmitter;

const BUFFER_SIZE: usize = 2;

pub enum TouchButtons {
	Left = 0x1,
	Down = 0x2,
	Up = 0x4,
	Right = 0x8,
	B = 0x10,
	A = 0x20
}

pub struct NetworkManager<'a> {
	phones: Vec<TcpStream>,
	remove_indices: Vec<usize>,
	rx: mpsc::Receiver<TcpStream>,
	bitmask_maps_down: Vec<HashMap<u8, (u32, Command)>>,
	bitmask_maps_up: Vec<HashMap<u8, (u32, Command)>>,
	command_emitter: &'a CommandEmitter<'a>
}


impl<'a> NetworkManager<'a> {
	//Start a thread to listen for incoming client connections
	pub fn begin_listening(cmdemit: &'a CommandEmitter) -> NetworkManager<'a> {
		let (tx, rx) = mpsc::channel();

		thread::spawn(move || {
			let listener = match TcpListener::bind("0.0.0.0:1337") {
				Ok(r) => {
					r
				}
				Err(e) => {
					panic!("Unable to bind TcpListener: {}", e);
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

		let mut maps_down = Vec::new();
		let mut maps_up = Vec::new();
		for _i in 0..::MAX_PLAYERS {
			maps_down.push(HashMap::new());
			maps_up.push(HashMap::new());
		}

		NetworkManager {
			phones: Vec::with_capacity(::MAX_PLAYERS),
			remove_indices: Vec::with_capacity(::MAX_PLAYERS),
			rx,
			bitmask_maps_down: maps_down,
			bitmask_maps_up: maps_up,
			command_emitter: cmdemit
		}
	}

	pub fn handle_input(&mut self) {
		//First thing to do is check if there are any pending connections
		match self.rx.try_recv() {
			Ok(stream) => {
				if self.phones.len() < ::MAX_PLAYERS {
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

					//Touchdown commands
					for j in 0..8 {
						match self.bitmask_maps_down[i].get_mut(&((1 << j) & buffer[0])) {
							Some((id, command)) => {
								self.command_emitter.emit_command(*id, *command);
							}
							None => { }
						}						
					}

					//Touchup commands
					for j in 0..8 {
						match self.bitmask_maps_up[i].get_mut(&((1 << j) & buffer[1])) {
							Some((id, command)) => {
								self.command_emitter.emit_command(*id, *command);
							}
							None => { }
						}
					}
				}
				Err(e) => {  }
			}
		}

		//Remove disconnected phones
		for i in self.remove_indices.iter() {
			self.phones.remove(*i);
		}
		self.remove_indices.clear();
	}

	pub fn add_touchdown_binding(&mut self, mask: u8, command: (u32, Command), player: usize) {
		self.bitmask_maps_down[player - 1].insert(mask, command);
	}

	pub fn add_touchup_binding(&mut self, mask: u8, command: (u32, Command), player: usize) {
		self.bitmask_maps_up[player - 1].insert(mask, command);
	}
}
