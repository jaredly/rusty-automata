
pub static NTEAMS: usize = 5;

pub fn sortCounts(counts: &mut [Count; 5]) {
  counts.sort_by(|a, b| {
    let bn = b.score();
    let an = a.score();
    return bn.cmp(&an);
  });
}

#[derive(Debug, Clone, Copy)]
pub struct Count {
  pub team: Team,

  pub sum: u8,
  pub num: u8,
  pub max: u8,
  pub greater: u8,
  // corners
  pub csum: u8,
  pub cnum: u8,
  pub cmax: u8,
  // manhatten
  pub msum: u8,
  pub mnum: u8,
  pub mmax: u8
}


impl Count {
  pub fn new() -> Count {
    Count {
      team:Team::Blank,

      sum:0,
      num:0,
      max:0,
      greater: 0,

      csum:0,
      cnum:0,
      cmax:0,

      msum:0,
      mnum:0,
      mmax:0
    }
  }
  pub fn score(&self) -> u8 {
    self.cnum + self.mnum * 2
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Team {
  Blank = 0,
  Blue = 1,
  Green = 2,
  Red = 3,
  Yellow = 4
}

pub fn nextTeam(team: Team) -> Team {
  match team {
    Team::Blue => Team::Green,
    Team::Green => Team::Red,
    Team::Red => Team::Yellow,
    Team::Yellow => Team::Blue,
    Team::Blank => Team::Red
  }
}

pub fn getRich(val: u8) -> (Team, u8) {
  match val {
     0     => (Team::Blank, 0),
     1...10 => (Team::Blue, val),
    11...20 => (Team::Green, val - 10),
    21...30 => (Team::Red, val - 20),
    31...40 => (Team::Yellow, val - 30),

    _      => (Team::Blank, 0)
  }
}

pub fn getPoor(team: Team, mut val: u8) -> u8 {
  if val > 10 {
    val = 10;
  }
  match team {
    Team::Blank => 0,
    Team::Blue => val,
    Team::Green => val + 10,
    Team::Red => val + 20,
    Team::Yellow => val + 30
  }
}

pub fn getTeam(val: u8) -> Team {
  match val {
     0     => Team::Blank,
     1...10 => Team::Blue,
    11...20 => Team::Green,
    21...30 => Team::Red,
    31...40 => Team::Yellow,

    _      => Team::Blank
  }
}

pub fn maketrix(width: usize, height: usize) -> Vec<Vec<u8>> {
  let mut matrix: Vec<Vec<u8>> = vec![];
  for _ in 0..height {
    let mut sub: Vec<u8> = vec![];
    for _ in 0..width {
      sub.push(0);
    }
    matrix.push(sub)
  }
  matrix
}

// get the predator team of the provided team
pub fn predator(team: Team) -> Team {
  match team {
    Team::Blank => Team::Blank,
    Team::Blue => Team::Yellow,
    Team::Green => Team::Blue,
    Team::Red => Team::Green,
    Team::Yellow => Team::Red
  }
}

// get the prey team of the given team
pub fn prey(team: Team) -> Team {
  match team {
    Team::Blank => Team::Blank,
    Team::Blue => Team::Green,
    Team::Green => Team::Red,
    Team::Red => Team::Yellow,
    Team::Yellow => Team::Blue
  }
}

/*
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
*/

