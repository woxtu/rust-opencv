use libc::{c_char, c_double, c_int, c_void};
use ffi::types::{CvCapture, CvMat, CvSize, CvVideoWriter, IplImage};

#[link(name = "opencv_highgui")]
extern "C" {
  pub fn cvCreateTrackbar2(trackbar_name: *const c_char,
                           window_name: *const c_char,
                           value: *const c_int,
                           count: c_int,
                           on_change: extern "C" fn (c_int, *const c_void) -> (),
                           userdata: *const c_void) -> c_int;
  pub fn cvGetTrackbarPos(trackbar_name: *const c_char, window_name: *const c_char) -> c_int;
  pub fn cvShowImage(name: *const c_char, image: *const IplImage) -> ();
  pub fn cvNamedWindow(name: *const c_char, flags: c_int) -> c_int;
  pub fn cvDestroyWindow(name: *const c_char) -> ();
  pub fn cvMoveWindow(name: *const c_char, x: c_int, y: c_int) -> ();
  pub fn cvResizeWindow(name: *const c_char, width: c_int, height: c_int) -> ();
  pub fn cvSetMouseCallback(window_name: *const c_char,
                            on_mouse: extern "C" fn (c_int, c_int, c_int, c_int, *const c_void) -> (),
                            param: *const c_void) -> ();
  pub fn cvSetTrackbarPos(trackbar_name: *const c_char, window_name: *const c_char, pos: c_int) -> ();
  pub fn cvWaitKey(delay: c_int) -> c_int;
  pub fn cvDecodeImage(buf: *const CvMat, iscolor: c_int) -> *const IplImage;
  pub fn cvEncodeImage(ext: *const c_char, image: *const CvMat, params: *const c_int) -> *const CvMat;
  pub fn cvLoadImage(filename: *const c_char, iscolor: c_int) -> *const IplImage;
  pub fn cvSaveImage(filename: *const c_char, image: *const IplImage, params: *const c_int) -> c_int;
  pub fn cvCaptureFromCAM(device: c_int) -> *const CvCapture;
  pub fn cvCaptureFromFile(filename: *const c_char) -> *const CvCapture;
  pub fn cvReleaseCapture(capture: *const *const CvCapture) -> ();
  pub fn cvGrabFrame(capture: *const CvCapture) -> c_int;
  pub fn cvRetrieveFrame(capture: *const CvCapture, streamIdx: c_int) -> *const IplImage;
  pub fn cvQueryFrame(capture: *const CvCapture) -> *const IplImage;
  pub fn cvGetCaptureProperty(capture: *const CvCapture, property_id: c_int) -> c_double;
  pub fn cvSetCaptureProperty(capture: *const CvCapture, property_id: c_int, value: c_double) -> c_int;
  pub fn cvCreateVideoWriter(filename: *const char, fourcc: c_int, fps: c_double, frame_size: CvSize, is_color: c_int) -> *const CvVideoWriter;
  pub fn cvReleaseVideoWriter(writer: *const *const CvVideoWriter) -> ();
  pub fn cvWriteFrame(writer: *const CvVideoWriter, image: *const IplImage) -> c_int;
}
