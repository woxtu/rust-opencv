use std::{mem, num, ptr};
use libc::{c_int, c_void};
use ffi::highgui::*;
use image::Image;

pub fn wait_key(delay: int) -> int {
  unsafe { cvWaitKey(delay as i32) as int }
}

pub struct Trackbar<'a> {
  name: String,
  window: String,
  on_change: Option<|uint|: 'a>,
}

impl<'a> Trackbar<'a> {
  pub fn position(&self) -> uint {
    self.name.with_c_str(|trackbar_name| {
      self.window.with_c_str(|window_name| unsafe {
        cvGetTrackbarPos(trackbar_name, window_name) as uint
      })
    })
  }

  pub fn set_position(&mut self, position: uint) {
    self.name.with_c_str(|trackbar_name| {
      self.window.with_c_str(|window_name| unsafe {
        cvSetTrackbarPos(trackbar_name, window_name, position as i32);
      })
    })
  }
}

impl<'a> Clone for Trackbar<'a> {
  fn clone(&self) -> Trackbar<'a> {
    Trackbar { name: self.name.clone(), window: self.window.clone(), on_change: None }
  }
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
  trackbars: Vec<Trackbar<'a>>,
  on_mouse: Option<|MouseEvent, int, int|: 'a>,
}

impl<'a> Window<'a> {
  pub fn named(name: &str) -> Result<Window, String> {
    name.with_c_str(|name_c_str| unsafe {
      match cvNamedWindow(name_c_str, 1i32) {
        0 => Ok(Window { name: name.to_string(), trackbars: Vec::new(), on_mouse: None }),
        _ => Err(name.to_string()),
      }
    })
  }

  pub fn show(&self, image: &Image) {
    self.name.with_c_str(|name| unsafe {
      cvShowImage(name, image.raw);
    })
  }

  pub fn move_(&self, x: int, y: int) {
    self.name.with_c_str(|name| unsafe {
      cvMoveWindow(name, x as i32, y as i32);
    })
  }

  pub fn resize(&self, width: int, height: int) {
    self.name.with_c_str(|name| unsafe {
      cvResizeWindow(name, width as i32, height as i32);
    })
  }

  pub fn create_trackbar(&mut self, name: &str, position: uint, max: uint, on_change: |position: uint|: 'a) -> Trackbar<'a> {
    extern "C" fn wrapper(pos: c_int, userdata: *const c_void) {
      let callback = unsafe { mem::transmute::<_, &mut |uint|>(userdata) };

      (*callback)(pos as uint);
    }

    self.trackbars.push(Trackbar { name: name.to_string(), window: self.name.clone(), on_change: Some(on_change) });
    let trackbar = self.trackbars.last().unwrap();

    trackbar.name.with_c_str(|trackbar_name| {
      trackbar.window.with_c_str(|window_name| unsafe {
        cvCreateTrackbar2(trackbar_name, window_name, ptr::null(), max as i32, wrapper, mem::transmute(trackbar.on_change.as_ref().unwrap()));
        cvSetTrackbarPos(trackbar_name, window_name, position as i32);
      })
    });

    trackbar.clone()
  }

  pub fn on_mouse(&mut self, on_mouse: |event: MouseEvent, x: int, y: int|: 'a) {
    extern "C" fn wrapper(event: c_int, x: c_int, y: c_int, _: c_int, param: *const c_void) {
      let event = num::from_i32::<MouseEvent>(event).unwrap();
      let callback = unsafe { mem::transmute::<_, &mut |MouseEvent, int, int|>(param) };

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
  fn drop(&mut self) {
    self.name.with_c_str(|name| unsafe {
      cvDestroyWindow(name);
    });
  }
}
