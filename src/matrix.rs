
use utils;

pub struct Matrix {
  pub values: Vec<Vec<u8>>,
  pub width: usize,
  pub height: usize
}

impl Matrix {
  pub fn fill(&mut self, x: isize, y: isize, w: isize, h: isize, val: u8) {
    for cx in x as usize..(x+w) as usize {
      for cy in y as usize..(y+h) as usize {
        self.values[cy][cx] = val;
      }
    }
  }
}

pub fn init(width: usize, height: usize) -> (Matrix, Matrix) {
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

