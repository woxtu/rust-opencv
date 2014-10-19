use libc::{c_char, c_double, c_int};
use ffi::types::{CvCapture, CvSize, CvVideoWriter, IplImage};

#[link(name = "opencv_video")]
extern "C" {
  pub fn cvCreateFileCapture(filename: *const c_char) -> *const CvCapture;
  pub fn cvCreateCameraCapture(index: c_int) -> *const CvCapture;
  pub fn cvGrabFrame(capture: *const CvCapture) -> c_int;
  pub fn cvRetrieveFrame(capture: *const CvCapture, streamIdx: c_int) -> *const IplImage;
  pub fn cvQueryFrame(capture: *const CvCapture) -> *const IplImage;
  pub fn cvReleaseCapture(capture: *const *const CvCapture) -> ();
  pub fn cvGetCaptureProperty(capture: *const CvCapture, property_id: c_int) -> c_double;
  pub fn cvSetCaptureProperty(capture: *const CvCapture, property_id: c_int, value: c_double) -> c_int;
  pub fn cvCreateVideoWriter(filename: *const c_char, fourcc: c_int, fps: c_double, frame_size: CvSize, is_color: c_int) -> *const CvVideoWriter;
  pub fn cvWriteFrame(writer: *const CvVideoWriter, image: *const IplImage) -> c_int;
  pub fn cvReleaseVideoWriter(writer: *const *const CvVideoWriter) -> ();
}
