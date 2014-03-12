extern crate sdl;
// extern crate sdl_ttf;

use matrix::{Matrix, init};
use sdl::video::{Color, RGB, Surface};
use utils::{Blank, Red, NTEAMS, Count};
use rules::Rules;

// use std::rand::Rng;
// use std::rand;

mod button;
mod utils;
mod patterns;
mod colors;
mod matrix;
mod rules;

struct Config {
  going: bool,
  width: uint,
  height: uint,
  theme: colors::Theme,
  pattern: patterns::Pattern,
  team: utils::Team
}

fn draw(config: &Config, screen: &sdl::video::Surface, mx: &Matrix) {
  let xscale = config.width as i16 / mx.width as i16;
  let yscale = config.height as i16/ mx.height as i16;
  for y in range(0u, mx.height) {
    for x in range(0u, mx.width) {
      screen.fill_rect(Some(sdl::Rect {
        x: (x as i16) * xscale,
        y: (y as i16) * yscale,
        w: xscale as u16,
        h: yscale as u16
      }), colors::colorize(config.theme, mx.values[y][x]));
    }
  }
}

fn initScreen(config: Config) -> ~Surface {
  match sdl::video::set_video_mode(
          config.width as int,
          config.height as int,
          32,
          [sdl::video::HWSurface],
          [sdl::video::DoubleBuf]) {
    Ok(screen) => screen,
    Err(err) => fail!("failed to set video mode: {}", err)
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

fn predates(one: u8, other: u8) -> bool {
  if other == 0 {return true}
  match one {
    0 => false,
    1 => other == 2,
    2 => other == 3,
    3 => other == 4,
    4 => other == 1,
    _ => false
  }
}

fn upBlank(rules: &Rules, current: &mut Matrix, x: uint, y: uint, counts: &mut [Count, ..NTEAMS]) {

  utils::sortCounts(counts);

  let mut i = match counts[0].team {
    Blank => 1,
    _ => 0
  };
  let score = counts[i].score();
  if score == counts[i+1].score() { // && score == counts[i+2].score() {
    current.values[y][x] = 0;
    return;
  }
    /*
  match utils::relationship(counts[i].team, counts[i+1].team) {
    utils::Neutral => {
      if counts[i].score() == counts[i+1].score() {
        current.values[y][x] = 0;
        return;
      }
    },
    utils::Prey => {
      let an = counts[i].score();
      let bn = counts[i+1].score();
      if an < bn + 3 {
        i += 1;
      }
    }
    _ => {}
  };
    */
  let count = &counts[i];

  if count.num > 0 {
    let nval = count.sum / count.num;
    if nval < 2 {
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
  } else if friends.greater >= 4 {
  // } else if friends.num >= rules.support && (friends.mmax > cval + 1 || friends.cmax > cval + 3) {
    1
  } else {
    0
  }
}

fn advance(rules: &Rules, old: &Matrix, current: &mut Matrix) {
  for x in range(0u, old.width) {
    for y in range(0u, old.height) {
      upOne(rules, old, current, x, y);
    }
  }
}

#[main]
pub fn main() {
  sdl::init([sdl::InitVideo]);
  sdl::wm::set_caption("Rust Simulator", "rust-sdl");

  let mut config = Config {
    width: 600,
    height: 600,
    theme: colors::Dark,
    pattern: patterns::Test,
    team: Red,
    going: false
  };

  let mut rules = Rules {
    danger: 1,
    crowd: 9,
    support: 5,
    alone: 3,
    food: 1,
    gang: false
  };

  // let mut rng = rand::rng();
  let screen = initScreen(config);
  // sdl_ttf::init();

  let (mut one, mut two) = init(200, 200);
  let mut old = &mut one;
  let mut current = &mut two;
  let mut third:&mut Matrix;

  let mut buttons: ~[button::Button] = ~[];
  buttons.push(button::Button {
    x: 10,
    y: 10,
    width: 60,
    height: 20,
    clicked: false,
    color: RGB(0, 255, 0),
    value: rules.crowd as int,
    action: rules::Crowd
  });

  /*
  let font = match sdl_ttf::open_font("./Gafata-Regular.ttf", 14) {
    Ok(loaded) => loaded,
    _ => fail!("Couldn't load the font")
  };
  */
  patterns::prefill(config.pattern, current);

  'main : loop {
    'event : loop {
      let thev = sdl::event::poll_event();
      let stop = buttons.mut_iter().any(|button| {
        if button.event(&thev) {
          rules::ruleIt(&mut rules, button.action, button.value as u8);
          return true;
        }
        false
      });
      if stop {
        break;
      }
      match thev {
        sdl::event::QuitEvent => break 'main,
        sdl::event::NoEvent => break 'event,
        sdl::event::KeyEvent(k, _, _, _)
                  if k == sdl::event::EscapeKey
                      => break 'main,
        sdl::event::KeyEvent(k, down, _, _) if down => {
          match k {
            // C: color change (mouse clicking)
            sdl::event::CKey => {
              config.team = utils::nextTeam(config.team)
            },
            // P: pause/play
            sdl::event::PKey => {
              config.going = !config.going
            },
            // T: theme change
            sdl::event::TKey => {
              config.theme = colors::nextTheme(config.theme);
            },
            // D: pattern change
            sdl::event::DKey => {
              config.pattern = patterns::nextPattern(config.pattern);
              old.fill(0,0,100,100,0);
              patterns::prefill(config.pattern, current)
            },
            sdl::event::SKey => {
              third = old;
              old = current;
              current = third;
              advance(&rules, old, current);
            },
            // SPACE: restart
            sdl::event::SpaceKey => {
              old.fill(0,0,100,100,0);
              patterns::prefill(config.pattern, current)
            },
            _ => {}
          }
        },
        sdl::event::MouseMotionEvent(st, x, y, _, _) => {
          if st.len() > 0 {
            current.values[y as uint * current.height / config.height][x as uint * current.width / config.width] = utils::getPoor(config.team, 10);
          }
        },
        _ => {}
      }
    }
    if config.going {
      third = old;
      old = current;
      current = third;
      advance(&rules, old, current);
    }
    draw(&config, screen, current);
    // for _ in buttons.iter().map(|b| b.draw(screen)) { }
    /*
    let text = match sdl_ttf::render_solid(font, "awesome", RGB(255, 0, 255)) {
      Ok(text) => text,
      _ => fail!("Couldn't draw string")
    };
    screen.blit(text);
    */

    screen.flip();
  }

  sdl::quit();
}

