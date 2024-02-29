//! Hello everyone! 
//! The purpose of this module is to provide the operating system API
//! and wrap it in a high-level language.
//! While that was the goal, 
//! I only implemented compatibility on Windows and wrappers for high-level languages.
//! The `mor` module contains the wrapper API, 
//! the `win` module contains the WindowsAPI for Windows system, 
//! and the `com` contains the compatible API. 
//! The difference between `mor` and `com` is that one is wrapped 
//! by using high-level language features, 
//! and the other is native API.
//! At present, only the wrapper of part of Windows file system API 
//! and the API reference of Windows file system and window system have been completed.


#![feature(decl_macro)]


pub mod def;
pub mod win;
pub mod com;
pub mod mor;
