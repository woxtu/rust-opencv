use std::mem;
use ffi::video::*;
use ffi::types::{CvCapture, CvSize, CvVideoWriter};
use core::Size;
use image::{BorrowedImage, Image};

pub struct Frames<'a> {
  capture: &'a Capture,
}

impl<'a> Frames<'a> {
  pub fn at(&self, index: uint) -> Option<Image> {
    unsafe {
      cvSetCaptureProperty(self.capture.raw, CV_CAP_PROP_POS_FRAMES, index as f64);
      match cvQueryFrame(self.capture.raw) {
        p if p.is_not_null() => Some(BorrowedImage(p)),
        _ => None,
      }
    }
  }

  pub fn count(&self) -> uint {
    unsafe { cvGetCaptureProperty(self.capture.raw, CV_CAP_PROP_FRAME_COUNT) as uint - 2 } // ??
  }
}

impl<'a> Iterator<Image> for Frames<'a> {
  fn next(&mut self) -> Option<Image> {
    let index = unsafe { cvGetCaptureProperty(self.capture.raw, CV_CAP_PROP_POS_FRAMES) as uint };
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
  fn drop(&mut self) {
    unsafe { cvReleaseCapture(&self.raw); }
  }
}

pub struct Writer {
  raw: *const CvVideoWriter,
}

impl Writer {
  pub fn open(path: &Path, fourcc: &[char, ..4], fps: f64, frame: &Size, is_color: bool) -> Result<Writer, String> {
    let fourcc = unsafe { mem::transmute::<_, i32>([fourcc[0] as u8, fourcc[1] as u8, fourcc[2] as u8, fourcc[3] as u8]) };
    let frame_size = CvSize { width: frame.width as i32, height: frame.height as i32 };
    let is_color = if is_color { 1i } else { 0i };

    path.with_c_str(|path| unsafe {
      match cvCreateVideoWriter(path, fourcc, fps as f64, frame_size, is_color as i32) {
        p if p.is_not_null() => Ok(Writer { raw: p }),
        _ => Err(path.to_string()),
      }
    })
  }

  pub fn write(&self, image: &Image) -> bool {
    unsafe { cvWriteFrame(self.raw, image.ptr()) != 0 }
  }
}

impl Drop for Writer {
  fn drop(&mut self) {
    unsafe { cvReleaseVideoWriter(&self.raw); }
  }
}
