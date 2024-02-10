/* **************************************
	File Name: Types
	Created: Saturday January 20 2024
*************************************** */
#![allow(non_snake_case)]
#![allow(dead_code)]
#![warn(unused_imports)]
#![allow(unused_parens)]
#![allow(non_camel_case_types)]

/* ********************************************************
	Imports
******************************************************** */

/* ********************************************************
	Enums & Structures
******************************************************** */

/* ********************************************************
	Public APIs
******************************************************** */

use std::ffi::c_void;

use serde::{Deserialize, Serialize};

pub type Callback = unsafe extern "C" fn(data: ffiString) -> bool;
/* ********************************************************
	Private APIs
******************************************************** */
#[derive(Default, Clone, Debug)]
#[repr(C)]
pub enum ffiDataType {
	#[default]
	UTF8_STRING = 0,
	ASCII_STRING,
	U8_BYTES,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ffiString {
	pub data_type: ffiDataType,
	pub ptr: *mut u8,
	pub length: usize,
	pub capacity: usize,
}

unsafe impl Send for ffiString {}

#[repr(C)]
pub struct userDataVoidPtr {
	userData: *mut c_void,
}
unsafe impl Send for userDataVoidPtr {}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub enum opCode {
	INIT,
	#[default]
	DEINIT,
	CFG,
	STATUS,
	EXIT,
}

pub enum internalMail {
	CMD(jsonCmdPkg),
	EVENT(jsonEventPkg, String),
	REGISTER_CALLBACKS(opCode, Callback),
	DEREGISTER_CALLBACKS(opCode),
	TOGGLE_DEBUG_MSG(bool),
	KILL_CMD,
}

#[derive(Debug)]
pub enum msgTypePkg<'a> {
	CMD(&'a opCode, &'a jsonCmdPkg),
	EVENT(&'a jsonEventPkg),
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
#[repr(C)]
pub enum msgType {
	CMD,
	#[default]
	EVENT,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct jsonCmdPkg {
	pub uid: String,
	pub resID: Option<String>,
	pub msgType: msgType,
	pub opCode: opCode,
	pub opRslt: Option<bool>,
	pub sessionID: Option<String>,
	pub data: Option<serde_json::Value>,
}
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct jsonEventPkg {
	pub uid: Option<String>,
	pub msgType: msgType,
	pub sessionID: Option<String>,
	pub eventData: Option<serde_json::Value>,
}
#[repr(C)]
pub struct voidData {
	pub data: [u32; 10],
	pub number: u32,
}
