extern crate sdl;
// extern crate sdl_ttf;

use matrix::{Matrix, init, Super};
use sdl::video::{RGB, Surface};
use utils::{Blank, Red, NTEAMS, Count};
use rules::Rules;
use config::Config;

// use std::rand::Rng;
// use std::rand;

mod automate;
mod button;
mod utils;
mod patterns;
mod colors;
mod matrix;
mod rules;
mod config;

fn handleKeys(k: sdl::event::Key, config: &mut Config, data: &mut Super) -> bool {
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
      data.pattern(config.pattern);
    },
    sdl::event::SKey => {
      return true;
    },
    // SPACE: restart
    sdl::event::SpaceKey => {
      data.pattern(config.pattern);
    },
    _ => {}
  }
  false
}

fn handleButtons(buttons: &mut [button::Button], thev: &sdl::event::Event, rules: &mut rules::Rules) -> bool {
  let stop = buttons.mut_iter().any(|button| {
    if button.event(thev) {
      rules::ruleIt(rules, button.action, button.value as u8);
      return true;
    }
    false
  });
  stop
}

pub fn run(config: &mut Config, rules: &mut Rules, buttons: &mut [button::Button]) {
  sdl::init([sdl::InitVideo]);
  sdl::wm::set_caption("Rust Simulator", "rust-sdl");

  let screen = config.initScreen();
  let mut data = Super::init(200, 200);
  // sdl_ttf::init();


  /*
  let font = match sdl_ttf::open_font("./Gafata-Regular.ttf", 14) {
    Ok(loaded) => loaded,
    _ => fail!("Couldn't load the font")
  };
  */
  patterns::prefill(config.pattern, data.current());

  'main : loop {
    'event : loop {
      let thev = sdl::event::poll_event();
      if handleButtons(buttons, &thev, rules) {
        break;
      }
      match thev {
        sdl::event::QuitEvent => break 'main,
        sdl::event::NoEvent => break 'event,
        sdl::event::KeyEvent(k, _, _, _)
                  if k == sdl::event::EscapeKey
                      => break 'main,
        sdl::event::KeyEvent(k, down, _, _) if down => {
          if handleKeys(k, config, &mut data) {
            data.flip();
            data.advance(rules);
          }
        },
        sdl::event::MouseMotionEvent(st, x, y, _, _) => {
          if st.len() > 0 {
            data.setPoint(y as uint * data.height / config.height, x as uint * data.width / config.width, utils::getPoor(config.team, 10));
          }
        },
        _ => {}
      }
    }
    if config.going {
      data.flip();
      data.advance(rules);
    }
    data.draw(config, screen);
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

#[main]
pub fn main() {
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
    min_grow: 2,
    gang: false
  };

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

  run(&mut config, &mut rules, buttons);
}

