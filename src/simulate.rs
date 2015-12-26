#![feature(augmented_assignments)]
extern crate sdl;
// extern crate sdl_ttf;
use matrix::{Matrix, init};
use sdl::video::{RGB, Surface};
use utils::{Team, NTEAMS, Count};
use rules::Rules;

// use std::rand::Rng;
// use std::rand;

mod button;
mod utils;
mod patterns;
mod colors;
mod matrix;
mod rules;

#[derive(Debug, Clone, Copy)]
struct Config {
  going: bool,
  width: usize,
  height: usize,
  theme: colors::Theme,
  pattern: patterns::Pattern,
  team: utils::Team
}

fn draw(config: &Config, screen: &sdl::video::Surface, mx: &Matrix) {
  let xscale = config.width as i16 / mx.width as i16;
  let yscale = config.height as i16/ mx.height as i16;
  let ref theme = config.theme;
  for y in 0usize..mx.height {
    for x in 0usize..mx.width {
      screen.fill_rect(Some(sdl::Rect {
        x: (x as i16) * xscale,
        y: (y as i16) * yscale,
        w: xscale as u16,
        h: yscale as u16
      }), colors::colorize(theme, mx.values[y][x]));
    }
  }
}

fn initScreen(config: Config) -> Surface {
  match sdl::video::set_video_mode(
          config.width as isize,
          config.height as isize,
          32,
          &[sdl::video::SurfaceFlag::HWSurface],
          &[sdl::video::VideoFlag::DoubleBuf]) {
    Ok(screen) => screen,
    Err(err) => panic!("failed to set video mode: {}", err)
  }
}


fn getCounts(old: &Matrix, x: usize, y: usize, cval: u8) -> [Count; 5] {
  let moves = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1), (0, -1)];
  let mut counts:[Count; 5] = [Count::new(); 5];
  for i in 0..8 {
    let (dx, dy) = moves[i];
    if dx + x < 0 ||
        dy + y < 0 ||
        dx + x >= old.width ||
        dy + y >= old.height {
      continue;
    }
    let (oteam, oval) = utils::getRich(old.values[dy+y][dx+x]);
    let count = &mut counts[oteam as usize];
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

fn upOne(rules: &Rules, old: &Matrix, current: &mut Matrix, x: usize, y: usize) {
  let (team, cval) = utils::getRich(old.values[y][x]);
  let mut counts = getCounts(old, x, y, cval);
  match team {
    Team::Blank => upBlank(rules, current, x, y, &mut counts),
    _ => upTeam(current, x, y, teamDiff(rules, team, &counts, cval), &counts, team, cval)
  }
}

fn upBlank(rules: &Rules, current: &mut Matrix, x: usize, y: usize, counts: &mut [Count; 5]) {

  utils::sortCounts(counts);

  let i = match counts[0].team {
    Team::Blank => 1,
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

fn upTeam(current: &mut Matrix, x: usize, y: usize, diff: i8, counts: &[Count; 5], team: utils::Team, cval: u8) {
  let now = cval as i8 + diff;
  current.values[y][x] = if now <= 0 {
    if counts[utils::predator(team) as usize].num > 0 && counts[utils::predator(team) as usize].max > 1 {
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

fn teamDiff(rules: &Rules, team: utils::Team, counts: &[Count; 5], cval: u8) -> i8 {
  // let empty = counts[Blank as usize] as i8;
  let food = counts[utils::prey(team) as usize];
  let danger = counts[utils::predator(team) as usize];
  let friends = counts[team as usize];
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
  // This makes things more round...but I think less isize
  // } else if friends.mmax > cval + 1 {
  //   1
  } else {
    0
  }
}

fn advance(rules: &Rules, old: &Matrix, current: &mut Matrix) {
  for x in 0usize..old.width {
    for y in 0usize..old.height {
      upOne(rules, old, current, x, y);
    }
  }
}

fn handleKeys(k: sdl::event::Key, config: &mut Config, current: &mut Matrix, old: &mut Matrix) -> bool {
  match k {
    // C: color change (mouse clicking)
    sdl::event::Key::C => {
      config.team = utils::nextTeam(config.team)
    },
    // P: pause/play
    sdl::event::Key::P => {
      config.going = !config.going
    },
    // T: theme change
    sdl::event::Key::T => {
      config.theme = colors::nextTheme(&config.theme);
    },
    // D: pattern change
    sdl::event::Key::D => {
      config.pattern = patterns::nextPattern(&config.pattern);
      old.fill(0,0,100,100,0);
      patterns::prefill(&config.pattern, current)
    },
    sdl::event::Key::S => {
      return true;
    },
    // SPACE: restart
    sdl::event::Key::Space => {
      old.fill(0,0,100,100,0);
      patterns::prefill(&config.pattern, current)
    },
    _ => {}
  }
  false
}

fn handleButtons(buttons: &mut Vec<button::Button>, thev: &sdl::event::Event, rules: &mut rules::Rules) -> bool {
  let stop = buttons.iter_mut().any(|button| {
    if button.event(thev) {
      rules::ruleIt(rules, &button.action, button.value as u8);
      return true;
    }
    false
  });
  stop
}

pub fn main() {
  sdl::init(&[sdl::InitFlag::Video]);
  sdl::wm::set_caption("Rust Simulator", "rust-sdl");

  let mut config = Config {
    width: 600,
    height: 600,
    theme: colors::Theme::Dark,
    pattern: patterns::Pattern::Test,
    team: Team::Red,
    going: false
  };

  let mut rules = Rules {
    danger: 1,
    crowd: 9,
    support: 5,
    alone: 3,
    food: 1,
    min_grow: 2,
    gang: false
  };

  let screen = initScreen(config);
  // sdl_ttf::init();

  let (mut one, mut two) = init(200, 200);
  let mut old = &mut one;
  let mut current = &mut two;
  let mut third:&mut Matrix;

  let mut buttons: Vec<button::Button> = vec![];
  buttons.push(button::Button {
    x: 10,
    y: 10,
    width: 60,
    height: 20,
    clicked: false,
    color: RGB(0, 255, 0),
    value: rules.crowd as isize,
    action: rules::RuleKey::Crowd
  });

  /*
  let font = match sdl_ttf::open_font("./Gafata-Regular.ttf", 14) {
    Ok(loaded) => loaded,
    _ => fail!("Couldn't load the font")
  };
  */
  patterns::prefill(&config.pattern, current);

  'main : loop {
    'event : loop {
      let thev = sdl::event::poll_event();
      if handleButtons(&mut buttons, &thev, &mut rules) {
        break;
      }
      match thev {
        sdl::event::Event::Quit => break 'main,
        sdl::event::Event::None => break 'event,
        sdl::event::Event::Key(k, _, _, _)
                  if k == sdl::event::Key::Escape
                      => break 'main,
        sdl::event::Event::Key(k, down, _, _) if down => {
          if handleKeys(k, &mut config, current, old) {
            third = old;
            old = current;
            current = third;
            advance(&rules, old, current);
          }
        },
        sdl::event::Event::MouseMotion(st, x, y, _, _) => {
          if st.len() > 0 {
            current.values[y as usize * current.height / config.height][x as usize * current.width / config.width] = utils::getPoor(config.team, 10);
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
    draw(&config, &screen, current);
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

