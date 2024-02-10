/* **************************************
	File Name:
	Created: Wednesday November 02 2022
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */
use super::appSettings;
use crate::GLOBALCFG;
use clap::{arg, value_parser, Command};
use std::path::PathBuf;

/* ********************************************************
	Enums & Structures
******************************************************** */
/* ********************************************************
	Public APIs
******************************************************** */
static APP_NAME: &str = "CLI APP";

/* ********************************************************
	Private APIs
******************************************************** */

pub fn startCmdLine() {
	let cmd_line = Command::new(APP_NAME)
		.version("0.1")
		.author("Yoofie <yoofie@gmail.com>")
		.about("CLI App v0.1 
		
This project exists to do something.")
		.arg(arg!(-i --input <INPUT_FILE> "The input file into this tool. This file should have been generated from the ddbug tool").value_parser(value_parser!(PathBuf)).default_value("output.json"))
		.arg(arg!(-d --rdbg "Prints out the internal Rust structures").value_parser(value_parser!(bool)).default_value("true"))
		.arg(arg!(-g --gui "GUI MODE").value_parser(value_parser!(bool)).default_value("false"))
		.get_matches();
	/* ********************************************************
		Get the data
	******************************************************** */
	let input_file = cmd_line.get_one::<PathBuf>("input");
	let rdbg = cmd_line.get_one::<bool>("rdbg");

	/* ********************************************************
		Welcome Message
	******************************************************** */
	if input_file.is_some() && rdbg.is_some() {
		println!(
			"Hello!\nUsing \"{}\" as input. DBG setting is \"{}\"\n",
			input_file.unwrap().to_string_lossy(),
			rdbg.unwrap()
		);
	}
	/* ********************************************************
		app Settings
	******************************************************** */
	let app: appSettings = appSettings {
		appName: APP_NAME.to_string(),
		appVersion: 0.1,
		cmdLine: cmd_line,
	};

	GLOBALCFG.set(app).expect("Failed to init global CFG");

	/* ********************************************************
		Some messages()
	******************************************************** */
	let ctx = GLOBALCFG.get().expect("Failed to init global CFG");
	println!("Hello, world! {} {}", ctx.appName, ctx.appVersion);
}
