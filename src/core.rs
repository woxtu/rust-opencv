#[deriving(Clone, PartialEq, Show)]
pub struct Color {
  red: u8,
  green: u8,
  blue: u8,
  alpha: u8,
}

impl Color {
  pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
    Color { red: red, green: green, blue: blue, alpha: 0 }
  }

  pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color { red: red, green: green, blue: blue, alpha: alpha }
  }

  pub fn as_scalar(&self) -> Scalar {
    [self.blue as f64, self.green as f64, self.red as f64, self.alpha as f64]
  }
}

#[deriving(Clone, PartialEq, Show)]
pub struct Point {
  pub x: int,
  pub y: int,
}

impl Point {
  pub fn new(x: int, y: int) -> Point {
    Point { x: x, y: y }
  }
}

#[deriving(Clone, PartialEq, Show)]
pub struct Rect {
  pub x: int,
  pub y: int,
  pub width: int,
  pub height: int,
}

impl Rect {
  pub fn new(x: int, y: int, width: int, height: int) -> Rect {
    Rect { x: x, y: y, width: width, height: height }
  }
}

pub type Scalar = [f64, ..4];

#[deriving(Clone, PartialEq, Show)]
pub struct Size {
  pub width: int,
  pub height: int,
}

impl Size {
  pub fn new(width: int, height: int) -> Size {
    Size { width: width, height: height }
  }
}
