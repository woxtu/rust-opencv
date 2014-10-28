use std::ptr;
use ffi::core::*;
use ffi::highgui::*;
use ffi::types::{AsCvArr, IplImage};
use core::Size;

pub enum Image {
  OwnedImage(*const IplImage),
  BorrowedImage(*const IplImage),
}

pub enum LoadColor {
  Unchanged = -1,
  GrayScale = 0,
  Color = 1,
}

impl Image {
  pub fn ptr(&self) -> *const IplImage {
    match *self {
      OwnedImage(p) => p,
      BorrowedImage(p) => p,
    }
  }

  pub fn load(path: &Path, flag: Option<LoadColor>) -> Result<Image, String> {
    path.with_c_str(|path_c_str| unsafe {
      match cvLoadImage(path_c_str, flag.unwrap_or(Color) as i32) {
        p if p.is_not_null() => Ok(OwnedImage(p)),
        _ => Err(path_c_str.to_string()),
      }
    })
  }

  pub fn save(&self, path: &Path) -> bool {
    path.with_c_str(|path| unsafe {
      cvSaveImage(path, self.ptr(), ptr::null()) == 0
    })
  }

  pub fn size(&self) -> Size {
    unsafe {
      let size = cvGetSize(self.ptr().as_arr());
      Size { width: size.width as int, height: size.height as int }
    }
  }

  pub fn width(&self) -> int { self.size().width }
  pub fn height(&self) -> int { self.size().height }
}

impl Clone for Image {
  fn clone(&self) -> Image {
    unsafe { OwnedImage(cvCloneImage(self.ptr())) }
  }
}

impl Drop for Image {
  fn drop(&mut self) {
    match *self {
      OwnedImage(p) => unsafe { cvReleaseImage(&p); },
      _ => (),
    }
  }
}
