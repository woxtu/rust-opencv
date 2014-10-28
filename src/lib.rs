#![crate_name = "opencv"]
#![crate_type = "lib"]
#![desc = "Wrapper for OpenCV"]
#![license = "MIT"]

#![feature(globs, unsafe_destructor)]
#![deny(unused_imports)]

extern crate libc;

pub mod core;
pub mod highgui;
pub mod image;
pub mod video;
mod ffi;
