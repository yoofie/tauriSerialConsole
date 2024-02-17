/* **************************************
	File Name: Async Engine
	Created: Sunday February 04 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::{thread::sleep, time::Duration};

use flume::Receiver;
use tauri::Manager;

/* ********************************************************
	Imports
******************************************************** */
use crate::{asyncEngine::ffiTypes::internalMail, TAURI_STATE};

pub mod ffiTypes;
/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */

pub async fn asyncEngineLoop(_rx: Receiver<internalMail>) {
	println!("ASYNC ENGINE RUNNING");
	//let (tx, rx): (Sender<internalMail>, Receiver<internalMail>) = loole::bounded(128);
	let tt = unsafe { TAURI_STATE.get().unwrap() };
	let window = tt
		.tauriEngine
		.clone()
		.unwrap()
		.0
		.read()
		.unwrap()
		.appHandle
		.clone()
		.unwrap();
	sleep(Duration::from_secs(2));

	let mut counter = 0;
	loop {
		//sleep(Duration::from_millis(100)).await;
		std::thread::sleep(std::time::Duration::from_secs(2));
		// emit a download progress event to all listeners registed in the webview
		let msg = format!("{{\"name\": \"EVENT_NAME\", \"value\": {}}}", counter);
		window.emit_all("wowza", msg).unwrap();
		counter += 1;
		if counter > 3 {
			println!("EXITING ASYNC LOOP!");
			break;
		}
	}
}
