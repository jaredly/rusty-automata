
use matrix::Matrix;

pub enum Pattern {
  Test,
  Cross,
  Line,
  Threes,
  Separate,
  Impasse,
  Center,
  Quadros,
  Diamond,
  Octag,
  Uneven,
  Morven
}

pub fn nextPattern(now: Pattern) -> Pattern {
  match now {
    Test => Cross,
    Cross => Line,
    Line => Threes,
    Threes => Separate,
    Separate => Impasse,
    Impasse => Center,
    Center => Quadros,
    Quadros => Diamond,
    Diamond => Octag,
    Octag => Uneven,
    Uneven => Morven,
    Morven => Test
  }
}

fn small_square(x: uint, y: uint, current: &mut Matrix) {
  current.values[x + 1][y] = 10;
  current.values[x][y] = 10;
  current.values[x][y + 1] = 10;

  current.values[x][y + 2] = 20;
  current.values[x][y + 3] = 20;
  current.values[x + 1][y + 3] = 20;

  current.values[x + 2][y + 3] = 30;
  current.values[x + 3][y + 3] = 30;
  current.values[x + 3][y + 2] = 30;

  current.values[x + 3][y + 1] = 40;
  current.values[x + 3][y] = 40;
  current.values[x + 2][y] = 40;
}

fn not_square(x: uint, y: uint, current: &mut Matrix) {
  current.values[x][y + 2] = 10;
  current.values[x + 1][y + 3] = 10;
  current.values[x + 2][y + 3] = 20;
  current.values[x + 3][y + 2] = 20;
  current.values[x + 3][y + 1] = 30;
  current.values[x + 2][y] = 30;
  current.values[x + 1][y] = 40;
  current.values[x][y + 1] = 40;
}

pub fn test(current: &mut Matrix) {

  small_square(20, 20, current);
  small_square(current.width - 24, current.height - 24, current);

}

pub fn prefill(pattern: Pattern, current: &mut Matrix) {
  current.fill(0, 0, current.width, current.height, 0);
  match pattern {
    Cross => cross(current),
    Line => line(current),
    Separate => separate(current),
    Threes => threes(current),
    Impasse => impasse(current),
    Test => test(current),
    Center => {
      let cx = current.width/ 2;
      let cy = current.height/ 2;
      current.fill(cx, cy, 10, 10, 10);
      current.fill(cx-10, cy, 10, 10, 20);
      current.fill(cx-10, cy-10, 10, 10, 30);
      current.fill(cx, cy-10, 10, 10, 40);
    },
    Quadros => {
      not_square(20, 20, current);
      not_square(current.width - 24, 20, current);
      not_square(current.width - 24, current.height - 24, current);
      not_square(20, current.height - 24, current);
    },
    Diamond => {
      not_square(current.width/2 - 2, 20, current);
      not_square(current.width/2 - 2, current.height - 24, current);
      not_square(20,                 current.height/2 - 2, current);
      not_square(current.width - 24, current.height/2 - 2, current);
    },
    Octag => {
      not_square(current.width/2 - 2, 20, current);
      not_square(current.width/2 - 2, current.height - 24, current);
      not_square(20,                 current.height/2 - 2, current);
      not_square(current.width - 24, current.height/2 - 2, current);

      not_square(20, 20, current);
      not_square(current.width - 24, 20, current);
      not_square(current.width - 24, current.height - 24, current);
      not_square(20, current.height - 24, current);
    },
    Uneven => {
      not_square(20, 20, current);
      not_square(76, 76, current);
      // not_square(current.width/2 - 20, current.height/2 - 20, current);
    },
    Morven => {
      not_square(20, 20, current);
      not_square(76, 70, current);
    }
  }
}

fn impasse(current: &mut Matrix) {
  current.fill(10, 12, 5, 16, 10);
  current.fill(16, 10, 5, 20, 20);
}

pub fn line(current: &mut Matrix) {
  let xs = current.width/20;
  let ys = current.height/20;
  for i in range(0u, 20) {
    current.fill(i * xs, i * ys, xs, ys, ((i % 4)*10 + 10) as u8)
  }
}

pub fn cross(current: &mut Matrix) {
  let xs = current.width/20;
  let ys = current.height/20;
  for i in range(0u, 20) {
    current.fill(i * xs, i * ys, xs, ys, ((i % 4)*10 + 10) as u8)
  }
  for i in range(0u, 20) {
    current.fill(i * xs, current.height- 10 - i * ys, xs, ys, ((i % 4)*10 + 10) as u8)
  }
  current.fill(current.width/2 - 10, current.height/2 - 10, 20, 20, 0);
  small_square(current.width/2 - 2, current.height/2 - 2, current);
}

pub fn threes(current: &mut Matrix) {
  current.fill(45, 45, 10, 10, 10);
  current.fill(55, 45, 10, 10, 20);
  current.fill(55, 55, 10, 10, 30);
  current.fill(45, 55, 10, 10, 40);
  // current.fill(60, 60, 10, 10, 10);
}

pub fn separate(current: &mut Matrix) {
  current.fill(5, 5, 10, 10, 10);
  current.fill(45, 45, 10, 10, 20);
  current.fill(85, 85, 10, 10, 30);
}

