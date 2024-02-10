/* **************************************
	File Name:{{project-name}} app settings
	Created: Wednesday November 02 2022
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

use std::sync::{Arc, RwLock};

/* ********************************************************
	Imports
******************************************************** */
use clap::ArgMatches;
pub use cmd::startCmdLine;
use tauri::AppHandle;

pub mod cmd;
/* ********************************************************
	Enums & Structures
******************************************************** */
#[derive(Debug)]
pub struct appSettings {
	pub appName: String,
	pub appVersion: f32,
	pub cmdLine: ArgMatches,
	//pub tauriState: Option<sWrapper>,
}

#[derive(Debug)]
pub struct tauriState {
	pub tauriEngine: Option<sWrapper>,
}

#[derive(Clone, Debug)]
pub struct sWrapper(pub Arc<RwLock<gState>>);

#[derive(Clone, Debug, Default)]
pub struct gState {
	pub appHandle: Option<AppHandle>,
	pub running: bool,
	pub configured: bool,
	pub addr: String,
}
/* ********************************************************
	Public APIs
******************************************************** */

/* ********************************************************
	Private APIs
******************************************************** */
impl gState {
	pub fn new() -> gState {
		gState {
			appHandle: None,
			running: false,
			configured: false,
			addr: "COM4".to_string(),
		}
	}

	pub fn set_handle(&mut self, handle: AppHandle) {
		self.appHandle = Some(handle);
	}
}

impl tauriState {
	pub fn new() -> tauriState {
		tauriState { tauriEngine: None }
	}
}
