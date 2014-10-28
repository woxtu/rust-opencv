#[deriving(Clone, PartialEq, Show)]
pub struct Color {
  red: f64,
  green: f64,
  blue: f64,
  alpha: f64,
}

impl Color {
  pub fn from_rgb(red: f64, green: f64, blue: f64) -> Color {
    Color { red: red, green: green, blue: blue, alpha: 0.0 }
  }

  pub fn from_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> Color {
    Color { red: red, green: green, blue: blue, alpha: alpha }
  }

  pub fn as_scalar(&self) -> Scalar {
    [self.blue, self.green, self.red, self.alpha]
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
