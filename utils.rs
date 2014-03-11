
pub enum Team {
  Blank = 0,
  Blue = 1,
  Green = 2,
  Red = 3,
  Yellow = 4
}

pub fn numTeam(val: u8) -> Team {
  match val {
    0 => Blank,
    1 => Blue,
    2 => Green,
    3 => Red,
    4 => Yellow,
    _ => Blank
  }
}

pub fn nextTeam(team: Team) -> Team {
  match team {
    Blue => Green,
    Green => Red,
    Red => Yellow,
    Yellow => Blue,
    Blank => Red
  }
}

pub fn getRich(val: u8) -> (Team, u8) {
  match val {
     0     => (Blank, 0),
     1..10 => (Blue, val),
    11..20 => (Green, val - 10),
    21..30 => (Red, val - 20),
    31..40 => (Yellow, val - 30),

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
    Red => val + 20,
    Yellow => val + 30
  }
}

pub fn getTeam(val: u8) -> Team {
  match val {
     0     => Blank,
     1..10 => Blue,
    11..20 => Green,
    21..30 => Red,
    31..40 => Yellow,

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

// get the predator team of the provided team
pub fn predator(team: Team) -> Team {
  match team {
    Blank => Blank,
    Blue => Yellow,
    Green => Blue,
    Red => Green,
    Yellow => Red
  }
}

// get the prey team of the given team
pub fn prey(team: Team) -> Team {
  match team {
    Blank => Blank,
    Blue => Green,
    Green => Red,
    Red => Yellow,
    Yellow => Blue
  }
}

pub enum Relationship {
  Predator,
  Prey,
  Neutral
}

pub fn relationship(t1: Team, t2: Team) -> Relationship {
  match t1 {
    Blank => Prey,
    Blue => match t2 {
      Blank => Predator,
    	Green => Predator,
      Yellow => Prey,
      _ => Neutral
    },
    Green => match t2 {
      Blank => Predator,
      Red => Predator,
      Blue => Prey,
      _ => Neutral
    },
    Red => match t2 {
      Blank => Predator,
      Yellow => Predator,
      Green => Prey,
      _ => Neutral
    },
    Yellow => match t2 {
      Blank => Predator,
      Blue => Predator,
      Red => Prey,
      _ => Neutral
    }
  }
}

