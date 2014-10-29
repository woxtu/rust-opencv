use std::ptr;
use ffi::core::*;
use ffi::highgui::*;
use ffi::imgproc::*;
use ffi::types::{AsCvArr, CvPoint, IplImage};
use core::{Color, Point, Size};

pub enum Image {
  OwnedImage(*const IplImage),
  BorrowedImage(*const IplImage),
}

impl Image {
  pub fn ptr(&self) -> *const IplImage {
    match *self {
      OwnedImage(p) => p,
      BorrowedImage(p) => p,
    }
  }

  pub fn load(path: &Path) -> Result<Image, String> {
    path.with_c_str(|path_c_str| unsafe {
      match cvLoadImage(path_c_str, 1) { // CV_LOAD_IMAGE_COLOR
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

  pub fn draw_line(&mut self, p1: &Point, p2: &Point, color: &Color, thickness: uint) {
    let p1 = CvPoint { x: p1.x as i32, y: p1.y as i32 };
    let p2 = CvPoint { x: p2.x as i32, y: p2.y as i32 };
    unsafe {
      cvLine(self.ptr().as_arr(), p1, p2, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }

  pub fn draw_circle(&mut self, center: &Point, radius: uint, color: &Color, thickness: uint) {
    let center = CvPoint { x: center.x as i32, y: center.y as i32 };
    unsafe {
      cvCircle(self.ptr().as_arr(), center, radius as i32, color.as_scalar(), thickness as i32, 16, 0); // CV_AA
    }
  }
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
