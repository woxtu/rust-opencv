use ffi::types::{CvArr, CvMat, CvSize, IplImage};

#[link(name = "opencv_core")]
extern "C" {
  pub fn cvCloneImage(image: *const IplImage) -> *const IplImage;
  pub fn cvCloneMat(mat: *const CvMat) -> *const CvMat;
  pub fn cvGetSize(mat: *const CvArr) -> CvSize;
  pub fn cvReleaseImage(image: *const *const IplImage);
  pub fn cvReleaseMat(mat: *const *const CvMat);
}
