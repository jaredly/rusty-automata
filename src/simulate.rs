extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::rect::Rect;

use matrix::{Matrix, init};
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

fn draw(config: &Config, screen: &mut Renderer, mx: &Matrix) {
  let xscale = config.width as i32 / mx.width as i32;
  let yscale = config.height as i32/ mx.height as i32;
  let ref theme = config.theme;
  for y in 0usize..mx.height {
    for x in 0usize..mx.width {
    screen.set_draw_color(colors::colorize(theme, mx.values[y][x]));
      screen.fill_rect(Rect::new(
        (x as i32) * xscale,
        (y as i32) * yscale,
        xscale as u32,
        yscale as u32
      ).unwrap().unwrap());
    }
  }
}

fn getCounts(old: &Matrix, x: usize, y: usize, cval: u8) -> [Count; 5] {
  let moves: [(isize, isize); 8] = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1), (0, -1)];
  let mut counts:[Count; 5] = [Count::new(); 5];
  let xi = x as isize;
  let yi = y as isize;
  for i in 0..8 {
    let (dx, dy) = moves[i];
    if dx + xi < 0 ||
        dy + yi < 0 ||
        dx + xi >= old.width as isize ||
        dy + yi >= old.height as isize {
      continue;
    }
    let (oteam, oval) = utils::getRich(old.values[(dy+yi) as usize][(dx+xi) as usize]);
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

fn handleKeys(k: sdl2::keyboard::Keycode, config: &mut Config, current: &mut Matrix, old: &mut Matrix) -> bool {
  match k {
    // C: color change (mouse clicking)
    Keycode::C => {
      config.team = utils::nextTeam(config.team)
    },
    // P: pause/play
    Keycode::P => {
      config.going = !config.going
    },
    // T: theme change
    Keycode::T => {
      config.theme = colors::nextTheme(&config.theme);
    },
    // D: pattern change
    Keycode::D => {
      config.pattern = patterns::nextPattern(&config.pattern);
      old.fill(0,0,100,100,0);
      patterns::prefill(&config.pattern, current)
    },
    Keycode::S => {
      return true;
    },
    // SPACE: restart
    Keycode::Space => {
      old.fill(0,0,100,100,0);
      patterns::prefill(&config.pattern, current)
    },
    _ => {}
  }
  false
}

/*
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
*/

pub fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let mut config = Config {
    width: 600,
    height: 600,
    theme: colors::Theme::Dark,
    pattern: patterns::Pattern::Test,
    team: Team::Red,
    going: true
  };

  let mut events = sdl_context.event_pump().unwrap();
  let window = video_subsystem.window("rust-sdl2 demo: Video", config.width as u32, config.height as u32)
    .position_centered()
    .build()
    .unwrap();
  let mut renderer = window.renderer().build().unwrap();

  let mut rules = Rules {
    danger: 1,
    crowd: 9,
    support: 5,
    alone: 3,
    food: 1,
    min_grow: 2,
    gang: false
  };

  // let screen = initScreen(config);
  // sdl_ttf::init();

  let (mut one, mut two) = init(200, 200);
  let mut old = &mut one;
  let mut current = &mut two;

  let mut buttons: Vec<button::Button> = vec![];
  buttons.push(button::Button {
    x: 10,
    y: 10,
    width: 60,
    height: 20,
    clicked: false,
    color: Color::RGB(0, 255, 0),
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
    for thev in events.poll_iter() {
      /*
      if handleButtons(&mut buttons, &thev, &mut rules) {
        break;
      }
      */
      match thev {
        sdl2::event::Event::Quit {..} => break 'main,
        sdl2::event::Event::KeyDown {keycode, ..}
                  if keycode == Some(Keycode::Escape)
                      => break 'main,
        sdl2::event::Event::KeyDown {keycode, ..} if keycode.is_some() => {
          if handleKeys(keycode.unwrap(), &mut config, current, old) {
            std::mem::swap(current, old);
            advance(&rules, old, current);
          }
        },
        sdl2::event::Event::MouseMotion {mousestate, x, y, ..} if mousestate.left() => {
          current.values[y as usize * current.height / config.height][x as usize * current.width / config.width] = utils::getPoor(config.team, 10);
        },
        _ => {}
      }
    }
    if config.going {
      std::mem::swap(current, old);
      advance(&rules, old, current);
    }
    draw(&config, &mut renderer, current);
    // for _ in buttons.iter().map(|b| b.draw(screen)) { }
    /*
    let text = match sdl_ttf::render_solid(font, "awesome", Color::RGB(255, 0, 255)) {
      Ok(text) => text,
      _ => fail!("Couldn't draw string")
    };
    screen.blit(text);
    */

    renderer.present();
  }

  //sdl2::quit();
}

