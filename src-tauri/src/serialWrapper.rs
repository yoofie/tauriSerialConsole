/* **************************************
	File Name: Serial Wrapper
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

use std::{
	default,
	fmt::Display,
	thread::{self, JoinHandle},
};

use loole::{Receiver, Sender};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{currentSerialStatus, serial::serial};

/* ********************************************************
	Enums & Structures
******************************************************** */
#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum frameType {
	ASCII,
	COBS,
	CUSTOM,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum serialCtrl {
	PLAY,
	PAUSE,
	EXIT,
	EXIT_THREAD,
	SERIAL_EXIT,
	NEW(serialSettings),
}
#[derive(Debug)]
pub enum message {
	BYTES(Vec<u8>),
	ASCII(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct serialSettings {
	pub parity: u8,
	pub stop_bits: u8,
	pub port_name: String,
	pub baud_rate: u32,
	pub decoder: frameType,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum serialState {
	#[default]
	IDLE,
	RUNNING,
}
/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Debug)]
pub struct sManager {
	pub tx: Sender<serialCtrl>,
	pub rx: Receiver<serialCtrl>,
	pub send_target: Option<Sender<bool>>,
	pub thread_handle: Option<JoinHandle<()>>,
	pub tauri_handle: Option<AppHandle>,
	pub serial_settings: Option<serialSettings>,
	pub sState: serialState,
}

#[derive(Debug)]
pub struct sCtrl {
	pub tx: Sender<serialCtrl>,
	pub thread_handle: Option<JoinHandle<()>>,
	pub tauri_handle: Option<AppHandle>,
}
/* ********************************************************
	Private APIs
******************************************************** */
impl sManager {
	pub fn new(handle: AppHandle) -> (sManager, sCtrl) {
		let (tx, rx) = loole::unbounded::<serialCtrl>();
		(
			sManager {
				tx: tx.clone(),
				rx: rx,
				send_target: None,
				thread_handle: None,
				tauri_handle: Some(handle.clone()),
				serial_settings: None,
				sState: serialState::IDLE,
			},
			sCtrl {
				tx,
				thread_handle: None,
				tauri_handle: Some(handle),
			},
		)
	}

	pub fn ctrl_loop(&mut self) {
		println!("CTRL RX THREAD");
		loop {
			match self.rx.recv() {
				Ok(cmd) => match cmd {
					serialCtrl::PLAY => {
						if self.sState == serialState::IDLE {
							println!("RX'd PLAY CMD");
							if self.serial_settings.is_some() {
								self.tx
									.send(serialCtrl::NEW(
										self.serial_settings
											.clone()
											.expect("#10 | FAILED TO GET SERIAL SETTINGS")
											.clone(),
									))
									.unwrap();
							} else {
								println!("FAILED PLAY CMD - SERIAL SERIAL SETTINGS MISSING");
							}
						}
					}

					serialCtrl::PAUSE => {
						if self.sState == serialState::RUNNING {
							println!("RX'd PAUSE COMMAND");
							if let Some(serial_tx) = self.send_target.clone() {
								serial_tx.send(true).expect("Failed to send serial kill command");
								self.send_target = None;
								self.thread_handle = None;
								self.sState = serialState::IDLE;
								self.updateSerialState();
							}
						}
					}

					serialCtrl::EXIT => {
						if self.sState == serialState::RUNNING {
							println!("RX'd EXIT CMD");
							if let Some(serial_tx) = self.send_target.clone() {
								if let Err(e) = serial_tx.send(true) {
									println!("Failed to send serial kill command! {}", e);
								}
								self.send_target = None;
								self.thread_handle = None;
								self.serial_settings = None;
								self.sState = serialState::IDLE;
								self.updateSerialState();
							}
						}
					}

					serialCtrl::NEW(cfg) => {
						if self.sState == serialState::IDLE {
							println!("RX'd NEW CMD");
							if self.validate_settings(&cfg) {
								self.serial_settings = Some(cfg.clone());
								let (tx, _rx) = loole::unbounded::<message>();
								let id = rand::thread_rng().gen_range(0..255);
								let mut ss = serial::new(
									cfg,
									tx,
									id,
									self.tauri_handle.clone().expect("FAILED TO EXTRACT APP HANDLE").clone(),
									self.tx.clone(),
								);
								self.send_target = Some(ss.get_tx());
								let thread = thread::spawn(move || {
									ss.run_serial();
								});
								self.thread_handle = Some(thread);
								self.sState = serialState::RUNNING;
								self.updateSerialState();
							} else {
								println!("Failed to validate settings!");
								if let Some(handle) = self.tauri_handle.clone() {
									if handle
										.emit_all("threadCtrl", "This event is show in frontend!!!")
										.is_err()
									{
										println!("#20 | Failed to emit threadCtrl status event");
									}
								}
							}
						}
					}

					serialCtrl::EXIT_THREAD => {
						println!("Received kill thread command!");
						break;
					}

					serialCtrl::SERIAL_EXIT => {
						self.send_target = None;
						self.thread_handle = None;
						self.sState = serialState::IDLE;
						self.updateSerialState();
					}
				},
				Err(e) => {
					println!("sMan | {e}");
				}
			}
		}

		println!("Exiting control loop!");
	}
	pub fn validate_settings(&self, _settings: &serialSettings) -> bool {
		true
	}

	pub fn get_current_settings(&self) -> bool {
		if self.serial_settings.is_some() {
			let _settings = serde_json::to_string(
				&self
					.serial_settings
					.clone()
					.expect("#11 | FAILED TO GET SERIAL SETTINGS")
					.clone(),
			)
			.expect("FAILED TO SERILIZE");
			true
		} else {
			false
		}
	}

	/// This Tauri managed state is used to propogate the serial state back to the Tauri command API and therefore back to the UI
	fn updateSerialState(&self) {
		let handle = self.tauri_handle.clone().unwrap();
		let my_state: tauri::State<currentSerialStatus> = handle.state();
		let mut nt = my_state.0.lock().unwrap();
		*nt = self.sState.clone();
	}
}

/* ********************************************************
	Extras
******************************************************** */

impl Display for serialState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			serialState::IDLE => {
				write!(f, "IDLE STATE")
			}
			serialState::RUNNING => {
				write!(f, "RUNNING STATE")
			}
		}
	}
}
