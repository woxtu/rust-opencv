use std::mem;
use libc::{c_double, c_int};

#[repr(C)]
pub struct CvArr;

pub trait AsCvArr {
  fn as_arr(&self) -> *const CvArr;
}

#[repr(C)]
pub struct CvBox2D;

#[repr(C)]
pub struct CvCapture;

#[repr(C)]
pub struct CvChain;

#[repr(C)]
pub struct CvChainPtReader;

#[repr(C)]
pub struct CvConnectedComp;

#[repr(C)]
pub struct CvContour;

#[repr(C)]
pub struct CvContourScanner;

#[repr(C)]
pub struct CvFont;

#[repr(C)]
pub struct CvHaarClassifierCascade;

#[repr(C)]
pub struct CvHistogram;

#[repr(C)]
pub struct CvHuMoments;

#[repr(C)]
pub struct CvLineIterator;

#[repr(C)]
pub struct CvMat;

impl AsCvArr for *const CvMat {
  fn as_arr(&self) -> *const CvArr {
    unsafe { mem::transmute(*self) }
  }
}

#[repr(C)]
pub struct CvMemStorage;

#[repr(C)]
pub struct CvMoments;

#[repr(C)]
pub struct CvPoint {
  pub x: c_int,
  pub y: c_int,
}

#[repr(C)]
pub struct CvPoint2D32f;

#[repr(C)]
pub struct CvRect {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
}

#[repr(C)]
pub type CvScalar = [c_double, ..4];

#[repr(C)]
pub struct CvSeq {
  pub flags: c_int,
  pub header_size: c_int,
  pub h_prev: *mut CvSeq,
  pub h_next: *mut CvSeq,
  pub v_prev: *mut CvSeq,
  pub v_next: *mut CvSeq,
  pub total: c_int,
}

#[repr(C)]
pub struct CvSeqBlock;

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

impl AsCvArr for *const IplImage {
  fn as_arr(&self) -> *const CvArr {
    unsafe { mem::transmute(*self) }
  }
} 
