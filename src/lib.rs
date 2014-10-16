#![crate_name = "opencv"]
#![crate_type = "lib"]
#![desc = "Wrapper for OpenCV"]
#![license = "MIT"]

#![feature(globs, unsafe_destructor)]

extern crate libc;

pub mod highgui;
pub mod image;
mod ffi;
