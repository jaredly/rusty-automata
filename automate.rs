
use matrix::{Matrix, init, Super};
use sdl::video::{RGB, Surface};
use utils::{Blank, Red, NTEAMS, Count};
use rules::Rules;
use config::Config;
use utils;

pub fn advance(rules: &Rules, old: &Matrix, current: &mut Matrix) {
  for x in range(0u, old.width) {
    for y in range(0u, old.height) {
      upOne(rules, old, current, x, y);
    }
  }
}


fn getCounts(old: &Matrix, x: uint, y: uint, cval: u8) -> [Count, ..NTEAMS] {
  let moves = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1), (0, -1)];
  let mut counts:[Count, ..NTEAMS] = [Count::new(), ..NTEAMS];
  for i in range(0, 8) {
    let (dx, dy) = moves[i];
    if dx + x < 0 ||
        dy + y < 0 ||
        dx + x >= old.width ||
        dy + y >= old.height {
      continue;
    }
    let (oteam, oval) = utils::getRich(old.values[dy+y][dx+x]);
    let count = &mut counts[oteam as int];
    count.team = oteam;
    count.sum += oval;
    count.num += 1;
    if oval > count.max {
      count.max = oval;
    }
    if oval > cval {
      count.greater += 1;
    }
    match dx + dy {
      1 | -1 => { // straight
        count.msum += oval;
        count.mnum += 1;
        if oval > count.mmax {
          count.mmax = oval;
        }
      },
      _ => { // diagonal
        count.csum += oval;
        count.cnum += 1;
        if oval > count.cmax {
          count.cmax = oval;
        }
      }
    };
  }
  counts
}

fn upOne(rules: &Rules, old: &Matrix, current: &mut Matrix, x: uint, y: uint) {
  let (team, cval) = utils::getRich(old.values[y][x]);
  let mut counts = getCounts(old, x, y, cval);
  match team {
    Blank => upBlank(rules, current, x, y, &mut counts),
    _ => upTeam(current, x, y, teamDiff(rules, team, &counts, cval), &counts, team, cval)
  }
}

fn upBlank(rules: &Rules, current: &mut Matrix, x: uint, y: uint, counts: &mut [Count, ..NTEAMS]) {

  utils::sortCounts(counts);

  let i = match counts[0].team {
    Blank => 1,
    _ => 0
  };
  let score = counts[i].score();
  if score == counts[i+1].score() { // && score == counts[i+2].score() {
    current.values[y][x] = 0;
    return;
  }
  let count = &counts[i];

  if count.num > 0 {
    let nval = count.sum / count.num;
    if nval < rules.min_grow {
      current.values[y][x] = 0;
    } else {
      current.values[y][x] = utils::getPoor(count.team, nval - 1);
    }
  } else {
    current.values[y][x] = 0;
  }
}

fn upTeam(current: &mut Matrix, x: uint, y: uint, diff: i8, counts: &[Count, ..NTEAMS], team: utils::Team, cval: u8) {
  let now = cval as i8 + diff;
  current.values[y][x] = if now <= 0 {
    if counts[utils::predator(team) as int].num > 0 && counts[utils::predator(team) as int].max > 1 {
      utils::getPoor(utils::predator(team), 1)
    } else {
      0
    }
  } else if now > 10 {
    utils::getPoor(team, 10)
  } else {
    utils::getPoor(team, now as u8)
  };
}

fn teamDiff(rules: &Rules, team: utils::Team, counts: &[Count, ..NTEAMS], cval: u8) -> i8 {
  // let empty = counts[Blank as int] as i8;
  let food = counts[utils::prey(team) as int];
  let danger = counts[utils::predator(team) as int];
  let friends = counts[team as int];
  if (rules.gang && danger.num > friends.num) || danger.num >= rules.danger {
    -1
  } else if friends.num >= rules.crowd {
    -1
  } else if food.num >= rules.food {
    1
  } else if friends.num <= rules.alone {
    -1
  } else if friends.greater >= rules.support {
    1
  // This makes things more round...but I think less interesting
  // } else if friends.mmax > cval + 1 {
  //   1
  } else {
    0
  }
}


