
use matrix::Matrix;

pub enum Pattern {
  Test,
  Cross,
  Threes,
  Separate,
  Impasse
}

pub fn nextPattern(now: Pattern) -> Pattern {
  match now {
    Test => Cross,
    Cross => Threes,
    Threes => Separate,
    Separate => Impasse,
    Impasse => Test
  }
}

pub fn test(current: &mut Matrix) {
  current.values[21][20] = 10;
  current.values[20][20] = 10;
  current.values[20][21] = 10;

  current.values[20][22] = 20;
  current.values[20][23] = 20;
  current.values[21][23] = 20;

  current.values[22][23] = 30;
  current.values[23][23] = 30;
  current.values[23][22] = 30;

  current.values[23][21] = 40;
  current.values[23][20] = 40;
  current.values[22][20] = 40;


  current.values[70][70] = 10;
  current.values[71][71] = 10;
  current.values[72][71] = 20;
  current.values[73][70] = 20;
  current.values[73][69] = 30;
  current.values[72][68] = 30;
  current.values[71][68] = 40;
  current.values[70][69] = 40;

}

pub fn prefill(pattern: Pattern, current: &mut Matrix) {
  current.fill(0, 0, current.width as int, current.height as int, 0);
  match pattern {
    Cross => cross(current),
    Separate => separate(current),
    Threes => threes(current),
    Impasse => impasse(current),
    Test => test(current)
  }
}

fn impasse(current: &mut Matrix) {
  current.fill(10, 12, 5, 16, 10);
  current.fill(16, 10, 5, 20, 20);
}

pub fn cross(current: &mut Matrix) {
  let xs = (current.width/20) as int;
  let ys = (current.height/20) as int;
  for i in range(0, 20) {
    current.fill(i * xs, i * ys, xs, ys, ((i % 4)*10 + 10) as u8)
  }
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

