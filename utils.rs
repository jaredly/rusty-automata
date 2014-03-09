
pub enum Team {
  Blank = 0,
  Blue = 1,
  Green = 2,
  Red = 3
}

pub fn numTeam(val: u8) -> Team {
  match val {
    0 => Blank,
    1 => Blue,
    2 => Green,
    3 => Red,
    _ => Blank
  }
}

pub fn nextTeam(team: Team) -> Team {
  match team {
    Blue => Green,
    Green => Red,
    Red => Blue,
    Blank => Red
  }
}

pub fn getRich(val: u8) -> (Team, u8) {
  match val {
     0     => (Blank, 0),
     1..10 => (Blue, val),
    11..20 => (Green, val - 10),
    12..30 => (Red, val - 20),
    _      => (Blank, 0)
  }
}

pub fn getPoor(team: Team, mut val: u8) -> u8 {
  if val > 10 {
    val = 10;
  }
  match team {
    Blank => 0,
    Blue => val,
    Green => val + 10,
    Red => val + 20
  }
}

pub fn getTeam(val: u8) -> Team {
  match val {
     0     => Blank,
     1..10 => Blue,
    11..20 => Green,
    12..30 => Red,
    _      => Blank
  }
}

pub fn maketrix(width: uint, height: uint) -> ~[~[u8]] {
  let mut matrix: ~[~[u8]] = ~[];
  for _ in range(0, height) {
    let mut sub: ~[u8] = ~[];
    for _ in range(0, width) {
      sub.push(0);
    }
    matrix.push(sub)
  }
  matrix
}

pub fn predator(team: Team) -> Team {
  match team {
    Blank => Blank,
    Green => Blue,
    Blue => Red,
    Red => Green
  }
}

pub fn prey(team: Team) -> Team {
  match team {
    Blank => Blank,
    Green => Red,
    Blue => Green,
    Red => Blue
  }
}

