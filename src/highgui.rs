use std::{mem, num};
use libc::{c_int, c_void};
use ffi::highgui::*;
use image::Image;

pub fn wait_key(delay: int) -> int {
  unsafe { cvWaitKey(delay as i32) as int }
}

#[deriving(FromPrimitive)]
pub enum MouseEvent {
  MouseMove = 0,
  LeftButtonDown,
  RightButtonDown,
  MiddleButtonDown,
  LeftButtonUp,
  RightButtonUp,
  MiddleButtonUp,
  LeftButtonDoubleClick,
  RightButtonDoubleClick,
  MiddleButtonDoubleClick,
  MouseWheel,
  MouseHorizontalWheel,
}

pub struct Window<'a> {
  name: String,
  on_mouse: Option<|MouseEvent, int, int|: 'a -> ()>,
}

impl<'a> Window<'a> {
  pub fn named(name: &str) -> Result<Window, String> {
    name.with_c_str(|name_c_str| unsafe {
      match cvNamedWindow(name_c_str, 1i32) {
        0 => Ok(Window { name: name.to_string(), on_mouse: None }),
        _ => Err(name.to_string()),
      }
    })
  }

  pub fn show(&self, image: &Image) -> () {
    self.name.with_c_str(|name| unsafe {
      cvShowImage(name, image.ptr());
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

  pub fn on_mouse(&mut self, on_mouse: |event: MouseEvent, x: int, y: int|: 'a -> ()) -> () {
    extern "C" fn wrapper(event: c_int, x: c_int, y: c_int, _: c_int, param: *const c_void) -> () {
      let event = num::from_i32::<MouseEvent>(event).unwrap();
      let callback = unsafe {
        mem::transmute::<_, &mut |MouseEvent, int, int| -> ()>(param)
      };

      (*callback)(event, x as int, y as int);
    }

    self.on_mouse = Some(on_mouse);
    self.name.with_c_str(|name| unsafe {
      cvSetMouseCallback(name, wrapper, mem::transmute(self.on_mouse.as_ref().unwrap()));
    })
  }
}

#[unsafe_destructor]
impl<'a> Drop for Window<'a> {
  fn drop(&mut self) -> () {
    self.name.with_c_str(|name| unsafe {
      cvDestroyWindow(name);
    });
  }
}
