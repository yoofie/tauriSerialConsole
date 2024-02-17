/* **************************************
	File Name: {{project-name}}
	Created: Wednesday November 02 2022
mod asyncEngine;

*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/* ********************************************************
Imports
******************************************************** */
use appCfg::{appSettings, gState, startCmdLine, tauriState};
use flume::{Receiver, Sender};

use once_cell::sync::OnceCell;
use serde_json::json;
use serialWrapper::{sCtrl, sManager, serialCtrl, serialSettings};
use std::{
	path::PathBuf,
	sync::{Arc, RwLock},
	thread::{self, sleep, spawn},
	time::Duration,
};

use tauri::{AboutMetadata, CustomMenuItem, Manager, Menu, MenuItem, Submenu};

use crate::{
	appCfg::sWrapper,
	asyncEngine::{asyncEngineLoop, ffiTypes::internalMail},
};
pub mod appCfg;
pub mod asyncEngine;
pub mod serial;
pub mod serialWrapper;
/* ********************************************************
	Enums & Structures
******************************************************** */
static GLOBALCFG: OnceCell<appSettings> = OnceCell::new();
static mut TAURI_STATE: OnceCell<tauriState> = OnceCell::new();
static G_STATE: OnceCell<gState> = OnceCell::new();
static SERIAL_CTRL: OnceCell<sCtrl> = OnceCell::new();

//static PROJECT_DIR: Dir = include_dir!("../../../ui/views/");
/* ********************************************************
	Public APIs
******************************************************** */
#[derive(Clone, serde::Serialize)]
struct Payload {
	message: String,
}

/* ********************************************************
	Private APIs
******************************************************** */

// Prevents additional console window on Windows in release, DO NOT REMOVE!!

fn main() {
	startCmdLine();

	let gCfg = &GLOBALCFG.get().unwrap().cmdLine;
	let input = gCfg.get_one::<PathBuf>("input").unwrap();
	let gui = *gCfg.get_one::<bool>("gui").unwrap();

	println!("--> {}", input.to_string_lossy());
	println!("hello world from rust!");

	/* Run Tauri GUI Engine
		******************************************************** */
	if !gui {
		/* ********************************************************
			Link Tauri EngineRuntime to globally accessable state
		******************************************************** */
		let tauriEngineWrapper = sWrapper(Arc::new(RwLock::new(gState::new())));
		{
			let tauriState = tauriState {
				tauriEngine: Some(tauriEngineWrapper.clone()),
			};
			unsafe { TAURI_STATE.set(tauriState).expect("FAILED TO SET G CTX") };
		}
		/* ********************************************************
			Tauri
		******************************************************** */
		tauri::Builder::default()
			.invoke_handler(tauri::generate_handler![
				resultJson, set_status, send_cfg, send_cmd, ctrl_play, ctrl_pause
			])
			.setup(|app| {
				{
					let mut xx = tauriEngineWrapper.0.write().unwrap();
					xx.set_handle(app.handle().clone());
				}
				let (mut sMan, mut sCtrl) = sManager::new(app.handle().clone());

				let thread = thread::spawn(move || {
					sMan.ctrl_loop();
				});
				sCtrl.thread_handle = Some(thread);
				SERIAL_CTRL.set(sCtrl).expect("#1 | FAILED TO CREATE SERIAL_CTRL CTX");

				let (_tx, rx): (Sender<internalMail>, Receiver<internalMail>) = flume::bounded(128);
				app.manage(tauriEngineWrapper);

				tauri::async_runtime::spawn(async move {
					println!("HELLO WORLD!!");
				});
				tauri::async_runtime::spawn(asyncEngineLoop(rx));

				app.listen_global("a1", |handler| {
					println!(
						"This event is come from frontend!!!\n\n\t{}",
						handler.payload().unwrap()
					);
				});

				Ok(())
			})
			.on_page_load(|app, _ev| {
				// in this place we can only emit events to frontend

				// --- emit event to frontend
				app.emit_all("b1", "This event is show in frontend!!!").unwrap();

				()
			})
			.run(tauri::generate_context!())
			.expect("Error starting application");
	}

	println!("Exiting!");
}

#[tauri::command]
fn resultJson() -> String {
	"{\"name\": \"Markus\", \"value\": 5}".into()
}

#[tauri::command]
fn set_status(state: tauri::State<sWrapper>) -> String {
	let mut sg = state.0.write().unwrap();

	sg.configured = true;
	let theState = json!(
		{
			"configured": sg.configured,
			"running": sg.running,
			"addr": sg.addr,
		}
	);
	theState.to_string()
}

#[tauri::command]
fn testFile() -> String {
	let my_str = "fdgsdfgasdfgfdg";
	include_str!("../rustfmt.toml");
	my_str.to_owned()
}

#[tauri::command]
fn send_cmd(req: String) -> Result<(), String> {
	let Some(theContext) = SERIAL_CTRL.get() else {
		println!("SERVER_AGENT | Error #4: Failed to get context reference :(");
		return Err("FAILED TO GET SERIAL CONTEXT CONTROL".to_string());
	};

	match serde_json::from_str::<serialCtrl>(req.as_str()) {
		Ok(pkg) => {
			let tx = theContext.tx.clone();
			if let Err(e) = tx.send(pkg) {
				println!("FAILED SEND | {}", e);
			}
			Ok(())
		}
		Err(e) => {
			println!("asdasd");
			Err(format!("Failed to parse JSON {}", e))
		}
	}
}

#[tauri::command]
fn send_cfg(blah: String) -> Result<String, String> {
	let Some(theContext) = SERIAL_CTRL.get() else {
		println!("SERVER_AGENT | Error #4: Failed to get context reference :(");
		return Err("FAILED TO GET SERIAL CONTEXT CONTROL".to_string());
	};

	println!("RX BACKEND | {}", blah);

	let settings = serde_json::from_str::<serialSettings>(blah.as_str()).expect("FAILED TO PARSE JSON");

	let tx = theContext.tx.clone();
	if let Err(e) = tx.send(serialCtrl::NEW(settings)) {
		println!("FAILED SEND | {}", e);
	}

	Ok("Just a message".to_string())
}

#[tauri::command]
fn ctrl_play() -> Result<String, String> {
	let Some(theContext) = SERIAL_CTRL.get() else {
		println!("SERVER_AGENT | Error #4: Failed to get context reference :(");
		return Err("FAILED TO GET SERIAL CONTEXT CONTROL".to_string());
	};

	let tx = theContext.tx.clone();
	match tx.send(serialCtrl::PLAY) {
		Ok(_) => Ok("PLAY COMMAND sent successfully!".to_string()),
		Err(e) => Err(format!("PLAY COMMAND FAILED TO SEND {}", e)),
	}
}

#[tauri::command]
fn ctrl_pause() -> Result<String, String> {
	let Some(theContext) = SERIAL_CTRL.get() else {
		println!("SERVER_AGENT | Error #4: Failed to get context reference :(");
		return Err("FAILED TO GET SERIAL CONTEXT CONTROL".to_string());
	};

	let tx = theContext.tx.clone();
	match tx.send(serialCtrl::PAUSE) {
		Ok(_) => Ok("PAUSE COMMAND sent successfully!".to_string()),
		Err(e) => Err(format!("PAUSE COMMAND FAILED TO SEND {}", e)),
	}
}

/* ********************************************************
	Reference
******************************************************** */
// https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523
