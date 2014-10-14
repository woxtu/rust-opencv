use libc::c_void;
use ffi::types::{CvMat, CvSize};

#[link(name = "opencv_core")]
extern "C" {
  pub fn cvCloneMat(mat: *const CvMat) -> *const CvMat;
  pub fn cvGetSize(mat: *const CvMat) -> CvSize;
  pub fn cvReleaseMat(mat: *const *const CvMat) -> c_void;
}

