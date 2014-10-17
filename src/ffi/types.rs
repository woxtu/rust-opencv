use libc::c_int;

#[repr(C)]
pub struct CvCapture;

#[repr(C)]
pub struct CvMat;

#[repr(C)]
pub struct CvSize {
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
pub struct CvVideoWriter;

#[repr(C)]
pub struct IplImage;
