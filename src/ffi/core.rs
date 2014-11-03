use ffi::types::{CvArr, CvMat, CvMemStorage, CvSeq, CvSize, IplImage};
use libc::{c_char, c_int, c_schar, c_void};

#[link(name = "opencv_core")]
extern "C" {
  pub fn cvCloneImage(image: *const IplImage) -> *const IplImage;
  pub fn cvCloneMat(mat: *const CvMat) -> *const CvMat;
  pub fn cvCreateMemStorage(block_size: c_int) -> *mut CvMemStorage;
  pub fn cvGetSeqElem(seq: *const CvSeq, index: int) -> *mut c_schar;
  pub fn cvGetSize(mat: *const CvArr) -> CvSize;
  pub fn cvLoad(
    filename: *const c_char, 
    memstorage: *mut CvMemStorage,
    name: *const c_char,
    real_name: *const c_char
  ) -> *mut c_void;
  pub fn cvReleaseImage(image: *const *const IplImage);
  pub fn cvReleaseMat(mat: *const *const CvMat);
}
