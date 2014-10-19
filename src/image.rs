use std::{mem, ptr};
use ffi::core::*;
use ffi::highgui::*;
use ffi::types::{IplImage};

pub struct Image {
  pub raw: *const IplImage,
}

pub enum LoadColor {
  Unchanged = -1,
  GrayScale = 0,
  Color = 1,
}

impl Image {
  pub fn load(path: &Path, flag: Option<LoadColor>) -> Result<Image, String> {
    path.with_c_str(|path_c_str| unsafe {
      match cvLoadImage(path_c_str, flag.unwrap_or(Color) as i32) {
        p if p.is_not_null() => Ok(Image { raw: p }),
        _ => Err(path_c_str.to_string()),
      }
    })
  }

  pub fn save(&self, path: &Path) -> bool {
    path.with_c_str(|path| unsafe {
      cvSaveImage(path, self.raw, ptr::null()) == 0
    })
  }

  pub fn width(&self) -> int {
    unsafe {
      let size = cvGetSize(mem::transmute(self.raw));
      size.width as int
    }
  }

  pub fn height(&self) -> int {
    unsafe {
      let size = cvGetSize(mem::transmute(self.raw));
      size.height as int
    }
  }
}

impl Clone for Image {
  fn clone(&self) -> Image {
    unsafe { Image { raw: cvCloneImage(self.raw) } }
  }
}

impl Drop for Image {
  fn drop(&mut self) -> () {
    unsafe { cvReleaseImage(&self.raw); }
  }
}
