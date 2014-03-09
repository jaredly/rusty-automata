use sdl::video::{Color, RGB, Surface};
use utils;
use utils::{Blank, Blue, Green, Red};

pub enum Theme {
  Light,
  Dark
}

pub fn colorize(theme: Theme, val: u8) -> Color {
  match theme {
    Light => light(val),
    Dark  => dark(val)
  }
}

pub fn nextTheme(theme: Theme) -> Theme {
  match theme {
    Light => Dark,
    Dark => Light
  }
}

fn light(val: u8) -> Color {
  match utils::getTeam(val) {
    Blank => RGB(255,255,255),
    Blue  => RGB(val * 10, val * 10, 155 + val * 10),
    Green => {
      let v = val - 10;
      RGB(v * 10, 155 + v * 10, v * 10)
    },
    Red   => {
      let v = val - 20;
      RGB(155 + v * 10, v * 10, v * 10)
    }
  }
}

fn dark(val: u8) -> Color {
  match utils::getTeam(val) {
    Blank => RGB(0,0,0),
    Blue  => RGB(0, 0, val*10),
    Green => {
      let v = val - 10;
      RGB(0, v*10, 0)
    },
    Red   => {
      let v = val - 20;
      RGB(v*10, 0, 0)
    }
  }
}

