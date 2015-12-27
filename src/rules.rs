

pub static NTEAMS: usize = 5;

pub fn sortCounts(counts: &mut [Count; 5]) {
  counts.sort_by(|a, b| {
    let bn = b.score();
    let an = a.score();
    return bn.cmp(&an);
  });
}

#[derive(Debug, Copy, Clone)]
pub struct Count {
  team: Team,

  sum: u8,
  num: u8,
  max: u8,
  greater: u8,
  // corners
  csum: u8,
  cnum: u8,
  cmax: u8,
  // manhatten
  msum: u8,
  mnum: u8,
  mmax: u8
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

#[derive(Debug, Copy, Clone)]
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


pub struct Rules {
  pub danger: u8,
  pub crowd: u8,
  pub alone: u8,
  pub food: u8,
  pub support: u8,
  pub min_grow: u8,
  pub gang: bool
}

pub enum RuleKey {
  Danger,
  Crowd,
  Alone,
  Support,
  Food
}

pub fn ruleIt(rules: &mut Rules, key: &RuleKey, val: u8) {
  match key {
    &RuleKey::Danger => rules.danger = val,
    &RuleKey::Support => rules.support = val,
    &RuleKey::Crowd => rules.crowd = val,
    &RuleKey::Alone => rules.alone = val,
    &RuleKey::Food => rules.food = val
  }
}

