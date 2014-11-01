use ffi::types::{CvArr, CvHaarClassifierCascade, CvMemStorage, CvSeq, CvSize};
use libc::{c_int, c_double};

#[link(name = "opencv_objdetect")]
extern "C" {
  pub fn cvHaarDetectObjects(image: *const CvArr, cascade: *mut CvHaarClassifierCascade, storage: *mut CvMemStorage,
    scale_factor: c_double, min_neighbors: c_int, flags: c_int,
    min_size: CvSize, max_size: CvSize) -> *mut CvSeq;
}