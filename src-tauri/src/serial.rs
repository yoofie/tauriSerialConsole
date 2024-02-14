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

use std::time::Duration;

use loole::{Receiver, Sender};
use tauri::{AppHandle, Manager};

use crate::serialWrapper::{frameType, message, serialCtrl, serialSettings};

/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Debug)]
pub struct serial {
	pub txrx: (Sender<bool>, Receiver<bool>),
	iBuffer: Vec<u8>,
	cobsBuffer: Vec<u8>,
	pub send_target: Sender<message>,
	settings: serialSettings,
	fType: frameType,
	id: u8,
	tauriHandle: AppHandle,
	sManagerTx: Sender<serialCtrl>,
}
/* ********************************************************
	Private APIs
******************************************************** */
impl serial {
	pub fn new(
		cfg: serialSettings,
		send: Sender<message>,
		id: u8,
		handle: AppHandle,
		sMan: Sender<serialCtrl>,
	) -> serial {
		let decoder = cfg.decoder.clone();
		serial {
			txrx: loole::unbounded::<bool>(),
			iBuffer: Vec::with_capacity(2048),
			cobsBuffer: Vec::with_capacity(2048),
			send_target: send,
			settings: cfg,
			fType: decoder,
			id: id,
			tauriHandle: handle,
			sManagerTx: sMan,
		}
	}

	pub fn get_tx(&self) -> Sender<bool> {
		self.txrx.0.clone()
	}

	pub fn run_serial(&mut self) {
		println!("RUNNING SERIAL Thread!!");
		let sPort = serialport::new(self.settings.port_name.as_str(), self.settings.baud_rate)
			.timeout(Duration::from_millis(20))
			.open();
		dbg!(&self.settings);
		match sPort {
			Ok(ref port) => {
				println!(
					"Receiving data on {} at {} baud:",
					&self.settings.port_name, &self.settings.baud_rate
				);

				port.clear(serialport::ClearBuffer::All).unwrap_or_else(|x| {
					println!("Error clearing buffer {}", x);
				});
				match sPort {
					Ok(mut port) => {
						println!(
							"Receiving data on {} at {} baud:",
							&self.settings.port_name.clone(),
							&self.settings.baud_rate.clone()
						);

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

							match self.txrx.1.try_recv() {
								Ok(value) => {
									if value {
										break;
									}
								}
								Err(_) => {}
							}
						}
					}
					Err(e) => {
						eprintln!("Failed to open \"{}\". Error: {}", self.settings.port_name.clone(), e);
						//::std::process::exit(1);
					}
				}
			}
			Err(e) => {
				eprintln!("Failed to open \"{}\". Error: {}", self.settings.port_name.clone(), e);
				//::std::process::exit(1);
			}
		}
		if let Err(e) = self.sManagerTx.send(serialCtrl::EXIT) {
			println!("SERIAL THREAD FAIL | FAILED TO SEND!!! | {}", e);
		}
		println!("Exiting Serial Thread #{}", self.id);
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
						print!("\t");
						s = std::str::from_utf8(&item[..]).expect("invalid utf-8 sequence");
						print!("UTF8 | {}\n", s);
						if let Err(e) = self.tauriHandle.emit_all("serialEvent", s) {
							println!("SERIAL | FAILED TO EMIT EVENT DATA | {}", e);
						}
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
