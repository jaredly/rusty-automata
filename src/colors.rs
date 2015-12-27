use sdl2::pixels::Color;
use utils;
use utils::{Team};

#[derive(Debug, Copy, Clone)]
pub enum Theme {
  Light,
  Dark,
  Orange
}

pub fn colorize(theme: &Theme, val: u8) -> Color {
  match theme {
    &Theme::Light => light(val),
    &Theme::Dark  => dark(val),
    &Theme::Orange => orange(val)
  }
}

pub fn nextTheme(theme: &Theme) -> Theme {
  match theme {
    &Theme::Light => Theme::Dark,
    &Theme::Dark => Theme::Orange,
    &Theme::Orange => Theme::Light
  }
}

fn light(val: u8) -> Color {
  match utils::getTeam(val) {
    Team::Blank => Color::RGB(255,255,255),
    Team::Blue  => Color::RGB(255 - val * 20, 255 - val * 20, 255),
    Team::Green => {
      let v = val - 10;
      Color::RGB(255 - v * 20, 255, 255 - v * 20)
    },
    Team::Red   => {
      let v = val - 20;
      Color::RGB(255, 255 - v * 20, 255 - v * 20)
    },
    Team::Yellow => {
      let v = val - 30;
      Color::RGB(255, 255, 255 - v * 20)
    }
  }
}

fn dark(val: u8) -> Color {
  match utils::getTeam(val) {
    Team::Blank => Color::RGB(0,0,0),
    Team::Blue  => Color::RGB(0, 0, val*25),
    Team::Green => {
      let v = val - 10;
      Color::RGB(0, v*25, 0)
    },
    Team::Red   => {
      let v = val - 20;
      Color::RGB(v*10, 0, v*13)
    },
    Team::Yellow => {
      let v = val - 30;
      Color::RGB(v*25, v*25, 0)
    }
  }
}

fn orange(val: u8) -> Color {
  match utils::getTeam(val) {
    Team::Blank => Color::RGB(0,0,0),
    Team::Blue  => Color::RGB(val*13, val*8, 0),
    Team::Green => {
      let v = val - 10;
      Color::RGB(v*25, v*9, 0)
    },
    Team::Red   => {
      let v = val - 20;
      Color::RGB(v*13, v*1, 0)
    },
    Team::Yellow => {
      let v = val - 30;
      Color::RGB(v*25, v*25, 0)
    }
  }
}

