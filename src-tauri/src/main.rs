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
use include_dir::{include_dir, Dir};
use loole::{Receiver, Sender};
use once_cell::sync::OnceCell;
use serde_json::json;
use std::{
	path::PathBuf,
	sync::{Arc, RwLock},
	thread::{sleep, spawn},
	time::Duration,
};

use tauri::{AboutMetadata, AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu};

use crate::{
	appCfg::sWrapper,
	asyncEngine::{asyncEngineLoop, ffiTypes::internalMail},
};
pub mod appCfg;
pub mod asyncEngine;
/* ********************************************************
	Enums & Structures
******************************************************** */
static GLOBALCFG: OnceCell<appSettings> = OnceCell::new();
static mut TAURI_STATE: OnceCell<tauriState> = OnceCell::new();
static G_STATE: OnceCell<gState> = OnceCell::new();

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
		let quit = CustomMenuItem::new("quit".to_string(), "Quit");
		let close = CustomMenuItem::new("close".to_string(), "Close");
		let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));

		let mut aboutInfo = AboutMetadata::new();
		aboutInfo.version = Some("v1.0".to_string());
		let menu = Menu::new()
			.add_submenu(submenu)
			.add_native_item(MenuItem::About("About".to_string(), aboutInfo))
			.add_native_item(MenuItem::Separator)
			.add_item(CustomMenuItem::new("hide", "Hide"));

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
			.menu(menu)
			.on_menu_event(|event| match event.menu_item_id() {
				"quit" => {
					println!("PRESSED QUIT!!!");
					std::process::exit(0);
				}
				"close" => {
					println!("PRESSED QUIT #2!!!");
					event.window().close().unwrap();
				}
				_ => {}
			})
			.invoke_handler(tauri::generate_handler![
				greet,
				my_custom_command,
				my_custom_command_with_result_value,
				resultJson,
				fn_with_error_handling,
				get_status,
				set_status
			])
			.setup(|app| {
				{
					let mut xx = tauriEngineWrapper.0.write().unwrap();
					xx.set_handle(app.handle().clone());
				}
				let (_tx, rx): (Sender<internalMail>, Receiver<internalMail>) = loole::bounded(128);
				app.manage(tauriEngineWrapper);

				tauri::async_runtime::spawn(async move {
					println!("HELLO WORLD!!");
				});
				tauri::async_runtime::spawn(asyncEngineLoop(rx));

				// listen to the `event-name` (emitted on any window)
				/* let id = */
				app.listen_global("clickr", |event| {
					println!("got event-name with payload {:?}", event.payload());
				});
				// unlisten to the event using the `id` returned on the `listen_global` function
				// a `once_global` API is also exposed on the `App` struct
				//app.unlisten(id);

				app.listen_global("a1", |handler| {
					println!(
						"This event is come from frontend!!!\n\n\t{}",
						handler.payload().unwrap()
					);
				});
				// emit the `event-name` event to all webview windows on the frontend
				app.emit_all(
					"clicky",
					Payload {
						message: "Tauri is awesome! FROM RUST".into(),
					},
				)
				.unwrap();

				app.listen_global("fr_response", |_handler| {
					println!("I listened reponse from frontend to sended event from backend!!!");
				});

				Ok(())
			})
			.on_page_load(|app, _ev| {
				// in this place we can only emit events to frontend

				// --- emit event to frontend
				app.emit_all("b1", "This event is show in frontend!!!").unwrap();

				// -- Sleep some time before send next event
				let copy = app.clone();
				spawn(move || {
					sleep(Duration::from_secs(2));
					// 3 --- emit event to frontend
					copy.emit("c2", "This is a second event emitted from backend").unwrap();
				});

				// 2 --- emit event to frontend
				app.emit_all("c1", "This is third message send from backend!!!")
					.unwrap();
				app.listen_global("fr_response", |_handler| {
					println!("I listened reponse from frontend to sended event from backend!!!");
				});
				()
			})
			.run(tauri::generate_context!())
			.expect("Error starting application");
	}

	println!("Exiting!");
}

#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}!", name)
}

#[tauri::command]
fn my_custom_command(invoke_message: String) {
	println!("I was invoked from JS, with this message: {}", invoke_message);
}

#[tauri::command]
fn my_custom_command_with_result_value() -> String {
	//"<h1>Hello from Rust!</h1>\n<blockquote>This is a message</blockquote>".into()
	let data = include_str!("../../ui/views/testFile.html");
	data.into()
}

#[tauri::command]
fn resultJson() -> String {
	"{\"name\": \"Markus\", \"value\": 5}".into()
}

#[tauri::command]
fn get_status(state: tauri::State<gState>) -> String {
	let theState = json!(
		{
			"configured": state.configured,
			"running": state.running,
			"addr": state.addr,
		}
	);
	theState.to_string()
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
fn fn_with_error_handling(number: u32) -> Result<String, String> {
	if number % 2 == 0 {
		Ok("This worked!".into())
	} else {
		Err("ERROR RESULT: PROVIDED NUMBER IS ODD!!".into())
	}
	// If something fails
	// If it worked
}
/*
#[tauri::event]
fn on_clickr() {
	println!("Button clicked!");
}
 */
#[tauri::command]
fn testFile() -> String {
	let my_str = "fdgsdfgasdfgfdg";
	include_str!("../rustfmt.toml");
	my_str.to_owned()
}

/* ********************************************************
	Reference
******************************************************** */
// https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523
