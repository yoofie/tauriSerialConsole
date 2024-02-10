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

use loole::{Receiver, Sender};
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

pub async fn asyncEngineLoop(rx: Receiver<internalMail>) {
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
	/* loop {
		tauri::async_runtime::select! {
			Some(rslt) = rx.recv() => {
				match rslt {
					internalMail::CMD(_x) =>{
						println!("ASYNC RT | X | Server sent command w/o being connected!");
					}

					internalMail::EVENT(_data, _origString) => {
						println!("ASYNC RT | X | E#22: Server sent eventData w/o being connect!");
					}

					internalMail::TOGGLE_DEBUG_MSG(newValue) => {
						println!("ASYNC RT | DBG MESSAGE | {newValue}");
					}

					internalMail::REGISTER_CALLBACKS(cmd, fnPtr) => {
						match gctx.fnCallbacks.insert(cmd.clone(), fnPtr) {
							Some(_x) => {
								println!("Function callback for {cmd} successfully updated!");
							}
							None => {
								println!("Function callback for {cmd} successfully registered!");
							}
						};
						gctx.test();
					}

					internalMail::DEREGISTER_CALLBACKS(cmd) => {
						gctx.fnCallbacks.remove(&cmd);
					}

					internalMail::KILL_CMD => {
						println!("ASYNC RT | Kill CMD rx'd");
						exit = true;
					}
				}
			}
		}
	} */
	let mut counter = 0;
	loop {
		std::thread::sleep(std::time::Duration::from_secs(2));
		// emit a download progress event to all listeners registed in the webview
		let msg = format!("{{\"name\": \"EVENT_NAME\", \"value\": {}}}", counter);
		window.emit_all("wowza", msg).unwrap();
		counter += 1;
		if counter > 40 {
			println!("EXITING ASYNC LOOP!");
			break;
		}
	}
	()
}
