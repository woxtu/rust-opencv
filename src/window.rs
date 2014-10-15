use ffi::highgui::*;
use image::Image;

pub struct Window {
  name: String,
}

pub enum WindowMode {
  Normal = 0x00000000,
  AutoSize = 0x00000001,
}

impl Window {
  pub fn create(name: &str, flag: Option<WindowMode>) -> Option<Window> {
    name.with_c_str(|name_c_str| unsafe {
      match cvNamedWindow(name_c_str, flag.unwrap_or(AutoSize) as i32) {
        0 => Some(Window {name: name.to_string()}),
        _ => None,
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
