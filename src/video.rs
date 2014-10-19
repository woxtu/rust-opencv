use ffi::video::*;
use ffi::types::{CvCapture};

pub struct Capture {
  raw: *const CvCapture,
}

impl Capture {
  pub fn from_file(path: &Path) -> Result<Capture, String> {
    path.with_c_str(|path_c_str| unsafe {
      match cvCreateFileCapture(path_c_str) {
        p if p.is_not_null() => Ok(Capture { raw: p }),
        _ => Err(path_c_str.to_string()),
      }
    })
  }
}

impl Drop for Capture {
  fn drop(&mut self) -> () {
    unsafe { cvReleaseCapture(&self.raw); }
  }
}
