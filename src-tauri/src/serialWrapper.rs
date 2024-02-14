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
	thread::{self, JoinHandle},
	time::Duration,
};

use loole::{unbounded, Receiver, Sender};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::serial::serial;

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

					serialCtrl::PAUSE => {
						println!("RX'd PAUSE COMMAND");
						if let Some(serial_tx) = self.send_target.clone() {
							serial_tx.send(true).expect("Failed to send serial kill command");
							self.send_target = None;
							self.thread_handle = None;
						}
					}

					serialCtrl::EXIT => {
						println!("RX'd EXIT CMD");
						if let Some(serial_tx) = self.send_target.clone() {
							serial_tx.send(true).expect("Failed to send serial kill command");
							self.send_target = None;
							self.thread_handle = None;
							self.serial_settings = None;
						}
					}

					serialCtrl::NEW(cfg) => {
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
							);
							self.send_target = Some(ss.get_tx());
							let thread = thread::spawn(move || {
								ss.run_serial();
							});
							self.thread_handle = Some(thread);
						} else {
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

					serialCtrl::EXIT_THREAD => {
						println!("Received kill thread command!");
						break;
					}
				},
				Err(e) => {
					println!("{e}");
				}
			}
		}

		println!("Exiting control loop!");
	}
	pub fn validate_settings(&self, _settings: &serialSettings) -> bool {
		false
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
}
