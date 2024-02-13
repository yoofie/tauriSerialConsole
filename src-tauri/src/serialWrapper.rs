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

use std::{thread::JoinHandle, time::Duration};

use loole::{unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

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
	pub send_target: Option<Sender<message>>,
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
						}
					}
				}
				Err(e) => {
					println!("{e}");
				}
			}
		}
	}
}

pub fn validate_settings(settings: serialSettings) -> bool {
	false
}
