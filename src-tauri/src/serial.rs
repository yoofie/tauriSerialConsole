/* **************************************
	File Name: Serial Control
	Created: Monday February 12 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */

/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Debug)]
pub struct serial {
	pub txrx: (Sender<serialCtrl>, Receiver<serialCtrl>),
	baud: u32,
	port_name: String,
	iBuffer: Vec<u8>,
	cobsBuffer: Vec<u8>,
	pub send_target: Option<Sender<message>>,
	settings: Option<serialSettings>,
	fType: frameType,
}
/* ********************************************************
	Private APIs
******************************************************** */
impl serial {
	pub fn run_serial(&mut self) {
		let sPort = serialport::new(self.port_name.as_str(), self.baud)
			.timeout(Duration::from_millis(20))
			.open();

		match sPort {
			Ok(mut port) => {
				let mut serial_buf: Vec<u8> = vec![0; 1000];
				println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);

				port.clear(serialport::ClearBuffer::All).unwrap_or_else(|x| {
					println!("Error clearing buffer {}", x);
				});
				match sPort {
					Ok(mut port) => {
						println!("Receiving data on {} at {} baud:", &self.port_name, &self.baud);

						port.clear(serialport::ClearBuffer::All).unwrap_or_else(|x| {
							println!("Error clearing buffer {}", x);
						});
						self.iBuffer.clear();

						loop {
							match port.bytes_to_read() {
								Ok(value) if value > 0 => {
									println!("\nGot {} bytes", value);
									let _ = port.read_to_end(&mut self.iBuffer);
									self.iBuffer.iter().for_each(|f| print!("{:#x} ", f));
									println!("");
									self.read_bytes();

									self.iBuffer.clear();
								}
								Ok(_value) => {}
								Err(error) => {
									println!("Got errors!");
									match error.kind() {
										serialport::ErrorKind::NoDevice => {
											println!("ERROR #2 | NO DEVICE | {}", error.description);
										}
										serialport::ErrorKind::InvalidInput => {
											println!("ERROR #2 | INVALID INPUT | {}", error.description);
										}
										serialport::ErrorKind::Unknown => {
											println!("ERROR #2 | UNKNOWN | {}", error.description);
										}
										serialport::ErrorKind::Io(_) => {
											println!("ERROR #2 | I/O | {}", error.description);
										}
									}
								}
							}
							if self.exit {
								break;
							}
							match self.channel.1.try_recv() {
								Ok(value) => {
									if value {
										self.exit();
									}
								}
								Err(_) => {}
							}
						}
					}
					Err(e) => {
						eprintln!("Failed to open \"{}\". Error: {}", self.port_name, e);
						::std::process::exit(1);
					}
				}
			}
			Err(e) => {
				eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
				::std::process::exit(1);
			}
		}
	}

	pub fn read_bytes(&mut self) {
		match self.fType {
			frameType::ASCII => {
				println!("\n{:#^50}", " ASCII FRAME ");
				//dbg!(&self.iBuffer);
				let newFrame = self.iBuffer.split(|f| *f == 0xa);
				println!("");
				let mut s: &str;
				for (indexx, item) in newFrame.enumerate() {
					if !item.is_empty() {
						print!("{} | ", indexx);
						item.iter().for_each(|ff| print!("{:#x} ", ff));
						print!("\n");
						s = std::str::from_utf8(&item[..]).expect("invalid utf-8 sequence");
					}
				}
			}
			frameType::COBS => {
				println!("COBS FRAME");

				match cobs::decode_vec(self.iBuffer.as_slice()) {
					Ok(_rslt) => {
						/* println!(
							"\nSUCCESS DECODE: {} \n{:x?}\n{:x?}",
							rslt.len(),
							rslt,
							&inputBuffer[..number_of_bytes]
						); */
					}
					_ => {
						//println!("\nFAIL: {number_of_bytes}\n{:x?}", &inputBuffer[..number_of_bytes]);
					}
				}
			}
			frameType::CUSTOM => {
				println!("CUSTOM FRAME");
			}
		}
	}
}
