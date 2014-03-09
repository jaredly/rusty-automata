extern crate sdl;
use matrix::{Matrix, init};
use sdl::video::{Color, RGB, Surface};
use utils::{Blank, Blue, Green, Red};

// use std::rand::Rng;
// use std::rand;

// mod button;
mod utils;
mod patterns;
mod colors;
mod matrix;

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

fn upOne(old: &Matrix, current: &mut Matrix, x: uint, y: uint) {
  let moves = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1), (0, -1)];
  let mut counts:[[u8, ..2], ..4] = [[0,0],[0,0],[0,0],[0,0]];
  let (team, cval) = utils::getRich(old.values[y][x]);
  for i in range(0, 8) {
    let (dx, dy) = moves[i];
    if dx + x < 0 ||
        dy + y < 0 ||
        dx + x >= old.width ||
        dy + y >= old.height {
      continue;
    }
    let strength = match dx + dy {
      1 | -1 => 2, // straight
      _      => 1  // diagonal
    };
    let (oteam, oval) = utils::getRich(old.values[dy+y][dx+x]);
    counts[oteam as int][0] += strength * oval;
    counts[oteam as int][1] += 1;
  }
  match team {
    Blank => upBlank(current, x, y, &counts),
    _ => upTeam(current, x, y, teamDiff(team, &counts, cval), team, cval)
  }
}

fn upBlank(current: &mut Matrix, x: uint, y: uint, counts: &[[u8, ..2], ..4]) {
  let mut which: u8 = 0;
  let mut what: u8 = 0;
  let mut when: u8 = 0;
  for i in range(0 as u8, 4) {
    if counts[i][0] > what {
      what = counts[i][0];
      when = counts[i][1];
      which = i;
    }
  }
  //if what > 1 {
  if (when > 0) {
    let nval = what / when / 2;
    if nval < 2 {
      which = 0;
    }
    current.values[y][x] = utils::getPoor(utils::numTeam(which), nval - 1);
  }
  //}
}

fn upTeam(current: &mut Matrix, x: uint, y: uint, diff: i8, team: utils::Team, cval: u8) {
  let now = cval as i8 + diff;
  current.values[y][x] = if now <= 0 {
    0
  } else if now > 10 {
    utils::getPoor(team, 10)
  } else {
    utils::getPoor(team, now as u8)
  };
}

fn teamDiff(team: utils::Team, counts: &[[u8, ..2], ..4], cval: u8) -> i8 {
  // let empty = counts[Blank as int] as i8;
  let food = counts[utils::prey(team) as int];
  let danger = counts[utils::predator(team) as int];
  let friends = counts[team as int];
  if danger[1] > 0 {
    -1
  } else if friends[1] > 6 {
    -1
  } else if friends[1] < 2 {
    -1
  } else if food[1] > 0 {
    1
  } else {
    0
  }
  // 12
  /*
  if food + danger + friends > 5 {
    -1
  } else if danger > friends {
    -1// -danger + friends + food/2
  } else if friends > 3 {
    -1
  } else if food > 0 {
    1
  } else {
    0
  }
  */
}

fn advance(old: &Matrix, current: &mut Matrix) {
  for x in range(0u, old.width) {
    for y in range(0u, old.height) {
      upOne(old, current, x, y);
    }
  }
}

#[main]
pub fn main() {
  sdl::init([sdl::InitVideo]);
  sdl::wm::set_caption("Rust Simulator", "rust-sdl");

  let mut config = Config {
    width: 400,
    height: 400,
    theme: colors::Dark,
    pattern: patterns::Cross,
    team: Red,
    going: true
  };

  // let mut rng = rand::rng();
  let screen = initScreen(config);

  let (mut one, mut two) = init(100, 100);
  let mut old = &mut one;
  let mut current = &mut two;
  let mut third:&mut Matrix;

  patterns::prefill(config.pattern, current);

  let going = true;

  'main : loop {
    'event : loop {
      match sdl::event::poll_event() {
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
      advance(old, current);
    }
    draw(&config, screen, current);

    screen.flip();
  }

  sdl::quit();
}

