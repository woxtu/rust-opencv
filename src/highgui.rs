use ffi::highgui::*;
use image::Image;

pub fn wait_key(delay: int) -> int {
  unsafe { cvWaitKey(delay as i32) as int }
}

pub struct Window {
  name: String,
}

impl Window {
  pub fn named(name: &str) -> Result<Window, String> {
    name.with_c_str(|name_c_str| unsafe {
      match cvNamedWindow(name_c_str, 1i32) {
        0 => Ok(Window {name: name.to_string()}),
        _ => Err(name.to_string()),
      }
    })
  }

  pub fn show(&self, image: &Image) -> () {
    self.name.with_c_str(|name| unsafe {
      cvShowImage(name, image.raw);
    })
  }

  pub fn move_(&self, x: int, y: int) -> () {
    self.name.with_c_str(|name| unsafe {
      cvMoveWindow(name, x as i32, y as i32);
    })
  }

  pub fn resize(&self, width: int, height: int) -> () {
    self.name.with_c_str(|name| unsafe {
      cvResizeWindow(name, width as i32, height as i32);
    })
  }
}

impl Drop for Window {
  fn drop(&mut self) -> () {
    self.name.with_c_str(|name| unsafe {
      cvDestroyWindow(name);
    });
  }
}

