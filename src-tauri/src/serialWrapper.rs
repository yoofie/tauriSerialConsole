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
use tauri::AppHandle;

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
	pub baud: u32,
	pub decoder: frameType,
}
/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Debug)]
pub struct sCtrl {
	pub tx: Sender<serialCtrl>,
	pub rx: Receiver<serialCtrl>,
	pub send_target: Option<Sender<bool>>,
	pub thread_handle: Option<JoinHandle<()>>,
	pub tauri_handle: Option<AppHandle>,
}
/* ********************************************************
	Private APIs
******************************************************** */
impl sCtrl {
	pub fn new(handle: AppHandle) -> sCtrl {
		let (tx, rx) = loole::unbounded::<serialCtrl>();
		sCtrl {
			tx: tx,
			rx: rx,
			send_target: None,
			thread_handle: None,
			tauri_handle: Some(handle),
		}
	}

	pub fn ctrl_loop(&mut self) {
		loop {
			match self.rx.recv() {
				Ok(cmd) => {
					println!("RX THREAD");
					match cmd {
						serialCtrl::PLAY => {
							println!("sdfsdf");
						}
						serialCtrl::PAUSE => {
							println!("sdfsdf");
						}
						serialCtrl::EXIT => {
							println!("sdfsdf");
						}
						serialCtrl::NEW(cfg) => {
							println!("sdfsdf");
							if self.validate_settings(&cfg) {
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
							}
						}
					}
				}
				Err(e) => {
					println!("{e}");
				}
			}
		}
	}
	pub fn validate_settings(&self, settings: &serialSettings) -> bool {
		false
	}
}
