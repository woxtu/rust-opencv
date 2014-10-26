use libc::{c_char, c_int, c_void};
use ffi::types::{CvMat, IplImage};

#[link(name = "opencv_highgui")]
extern "C" {
  pub fn cvDestroyWindow(name: *const c_char);
  pub fn cvNamedWindow(name: *const c_char, flags: c_int) -> c_int;
  pub fn cvShowImage(name: *const c_char, image: *const IplImage);
  pub fn cvResizeWindow(name: *const c_char, width: c_int, height: c_int);
  pub fn cvMoveWindow(name: *const c_char, x: c_int, y: c_int);
  pub fn cvCreateTrackbar2(trackbar_name: *const c_char,
                           window_name: *const c_char,
                           value: *const c_int,
                           count: c_int,
                           on_change: extern "C" fn (pos: c_int, userdata: *const c_void),
                           userdata: *const c_void) -> c_int;
  pub fn cvGetTrackbarPos(trackbar_name: *const c_char, window_name: *const c_char) -> c_int;
  pub fn cvSetTrackbarPos(trackbar_name: *const c_char, window_name: *const c_char, pos: c_int);
  pub fn cvSetMouseCallback(window_name: *const c_char,
                            on_mouse: extern "C" fn (event: c_int, x: c_int, y: c_int, flags: c_int, param: *const c_void),
                            param: *const c_void);
  pub fn cvWaitKey(delay: c_int) -> c_int;

  pub fn cvDecodeImage(buf: *const CvMat, iscolor: c_int) -> *const IplImage;
  pub fn cvEncodeImage(ext: *const c_char, image: *const CvMat, params: *const c_int) -> *const CvMat;
  pub fn cvLoadImage(filename: *const c_char, iscolor: c_int) -> *const IplImage;
  pub fn cvSaveImage(filename: *const c_char, image: *const IplImage, params: *const c_int) -> c_int;
}
