use ffi::core::*;
use ffi::video::*;
use ffi::types::{CvCapture};
use image::Image;

pub struct Frames<'a> {
  capture: &'a Capture,
  index: uint,
  count: uint,
}

impl<'a> Iterator<Image> for Frames<'a> {
  fn next(&mut self) -> Option<Image> {
    if self.index < self.count {
      let result = unsafe {
        cvSetCaptureProperty(self.capture.raw, 1i32, self.index as f64);
        Image { raw: cvCloneImage(cvQueryFrame(self.capture.raw)) }
      };
      self.index = self.index + 1u;
      Some(result)
    } else {
      None
    }
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
    let count = unsafe {
      cvGetCaptureProperty(self.raw, 7i32)
    } as uint;
    Frames { capture: self, index: 0u, count: count }
  }
}

impl Drop for Capture {
  fn drop(&mut self) -> () {
    unsafe { cvReleaseCapture(&self.raw); }
  }
}
