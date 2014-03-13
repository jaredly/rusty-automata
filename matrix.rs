
use sdl::Rect;
use sdl::video::Surface;
use utils;
use config;
use colors;
use patterns;
use automate::advance;
use rules::Rules;

pub struct Matrix {
  values: ~[~[u8]],
  width: uint,
  height: uint
}

impl Matrix {
  pub fn fill(&mut self, x: uint, y: uint, w: uint, h: uint, val: u8) {
    for cx in range(x, x+w) {
      for cy in range(y, y+h) {
        self.values[cy][cx] = val;
      }
    }
  }

  pub fn draw(&self, config: &config::Config, screen: &Surface) {
    let xscale = config.width as i16 / self.width as i16;
    let yscale = config.height as i16/ self.height as i16;
    for y in range(0u, self.height) {
      for x in range(0u, self.width) {
        screen.fill_rect(Some(Rect {
          x: (x as i16) * xscale,
          y: (y as i16) * yscale,
          w: xscale as u16,
          h: yscale as u16
        }), colors::colorize(config.theme, self.values[y][x]));
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

pub struct Super {
  one: Matrix,
  two: Matrix,
  first: bool,
  height: uint,
  width: uint
  // old: &Matrix,
  // current: &Matrix
}

impl Super {
  pub fn init(width: uint, height: uint) -> Super {
    let (one, two) = init(width, height);
    // let mut old = &mut one;
    // let mut current = &mut two;
    // let mut third:&mut Matrix;
    Super {
      one: one,
      two: two,
      first: false,
      height: height,
      width: width
    }
  }

  pub fn pattern(&mut self, pattern: patterns::Pattern) {
    self.old().fill(0,0,self.width,self.height,0);
    patterns::prefill(pattern, self.current())
  }

  pub fn setPoint(&mut self, y: uint, x: uint, num: u8) {
    self.current().values[y][x] = num;
  }

  pub fn draw(&mut self, config: &config::Config, screen: &Surface) {
    self.current().draw(config, screen);
  }

  pub fn mxs<'r>(&'r mut self) -> (&'r mut Matrix, &'r Matrix) {
    match self.first {
      true => (&mut self.one, &self.two),
      false => (&mut self.two, &self.one)
    }
  }

  pub fn current<'r>(&'r mut self) -> &'r mut Matrix {
    match self.first {
      true => &mut self.one,
      false => &mut self.two
    }
  }
  pub fn old<'r>(&'r mut self) -> &'r mut Matrix {
    match self.first {
      true => &mut self.two,
      false => &mut self.one
    }
  }
  pub fn flip(&mut self) {
    self.first = !self.first
  }
  pub fn advance(&mut self, rules: &Rules) {
    let (current, old) = self.mxs();
    advance(rules, old, current);
  }
}

