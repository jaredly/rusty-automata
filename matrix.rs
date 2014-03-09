
use utils;

pub struct Matrix {
  values: ~[~[u8]],
  width: uint,
  height: uint
}

impl Matrix {
  pub fn fill(&mut self, x: int, y: int, w: int, h: int, val: u8) {
    for cx in range(x, x+w) {
      for cy in range(y, y+h) {
        self.values[cy][cx] = val;
      }
    }
  }
}

pub fn init(width: uint, height: uint) -> (Matrix, Matrix) {
  let one = Matrix {
    values: utils::maketrix(width, height),
    width: width,
    height: height
  };
  let two = Matrix {
    values: utils::maketrix(width, height),
    width: width,
    height: height
  };
  (one, two)
}

