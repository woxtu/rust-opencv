use ffi::highgui::*;

pub fn wait_key(delay: int) -> int {
  unsafe { cvWaitKey(delay as i32) as int }
}
