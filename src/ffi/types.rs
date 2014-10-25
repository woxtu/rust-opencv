use libc::c_int;

#[repr(C)]
pub struct CvArr;

#[repr(C)]
pub struct CvBox2D;

#[repr(C)]
pub struct CvCapture;

#[repr(C)]
pub struct CvChain;

#[repr(C)]
pub struct CvContour;

#[repr(C)]
pub struct CvHistogram;

#[repr(C)]
pub struct CvHuMoments;

#[repr(C)]
pub struct CvMat;

#[repr(C)]
pub struct CvMemStorage;

#[repr(C)]
pub struct CvMoments;

#[repr(C)]
pub struct CvPoint;

#[repr(C)]
pub struct CvPoint2D32f;

#[repr(C)]
pub struct CvRect;

#[repr(C)]
pub struct CvScalar;

#[repr(C)]
pub struct CvSeq;

#[repr(C)]
pub struct CvSize {
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
pub struct CvSlice;

#[repr(C)]
pub struct CvTermCriteria;

#[repr(C)]
pub struct CvVideoWriter;

#[repr(C)]
pub struct IplConvKernel;

#[repr(C)]
pub struct IplImage;
