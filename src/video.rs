use ffi::core::*;
use ffi::video::*;
use ffi::types::{CvCapture};
use image::Image;

pub struct Frames<'a> {
  capture: &'a Capture,
}

impl<'a> Frames<'a> {
  pub fn at(&self, index: uint) -> Option<Image> {
    unsafe {
      cvSetCaptureProperty(self.capture.raw, 1, index as f64);
      match cvQueryFrame(self.capture.raw) {
        p if p.is_not_null() => Some(Image { raw: cvCloneImage(p) }),
        _ => None,
      }
    }
  }

  pub fn count(&self) -> uint {
    unsafe { cvGetCaptureProperty(self.capture.raw, 7) as uint - 2 } // ??
  }
}

impl<'a> Iterator<Image> for Frames<'a> {
  fn next(&mut self) -> Option<Image> {
    let index = unsafe { cvGetCaptureProperty(self.capture.raw, 1) as uint };
    self.at(index + 1)
  }
}

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

  pub fn frames(&self) -> Frames {
    Frames { capture: self }
  }
}

impl Drop for Capture {
  fn drop(&mut self) -> () {
    unsafe { cvReleaseCapture(&self.raw); }
  }
}
