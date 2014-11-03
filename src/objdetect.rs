use ffi::types::{CvArr, CvHaarClassifierCascade, CvSize};
use std::ptr;
use core::{Size, Seq};
use image::{Image};
use ffi::objdetect::*;
use ffi::core::*;

pub struct CascadeClassifier {
  raw: *mut CvHaarClassifierCascade
}

impl CascadeClassifier {
  
  pub fn load(path: &Path) -> Result<CascadeClassifier, String> {
    path.with_c_str(|path_c_str| unsafe {
      match cvLoad(path_c_str, ptr::null_mut(), ptr::null(), ptr::null()) {
        c if c.is_not_null() => Ok(CascadeClassifier { raw: c as *mut CvHaarClassifierCascade }),
        _ => Err(path_c_str.to_string()),
      }
    })
  }

  pub fn detect_multi_scale(&self, image: &Image, 
    scale_factor: f64, min_neighbors: int, flags: int, 
    min_size: Size, max_size: Size) -> Result<Seq, String> {

    unsafe {
      match cvHaarDetectObjects(
        image.ptr() as *const CvArr, 
        self.raw,
        cvCreateMemStorage(0),
        scale_factor,
        min_neighbors as i32,
        flags as i32,
        CvSize { width: min_size.width as i32, height: min_size.height as i32 },
        CvSize { width: max_size.width as i32, height: max_size.height as i32 }
      ) {
        r if r.is_not_null() => Ok(Seq { raw: r, curr: 0u }),
        _ => Err("Something went wrong!".to_string())
      }
    }

  }

}