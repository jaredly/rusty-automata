use sdl::video::{Color, RGB, Surface};
use utils;
use utils::{Blank, Blue, Green, Red, Yellow};

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
    Blue  => RGB(255 - val * 20, 255 - val * 20, 255),
    Green => {
      let v = val - 10;
      RGB(255 - v * 20, 255, 255 - v * 20)
    },
    Red   => {
      let v = val - 20;
      RGB(255, 255 - v * 20, 255 - v * 20)
    },
    Yellow => {
      let v = val - 30;
      RGB(255, 255, 255 - v * 20)
    }
  }
}

fn dark(val: u8) -> Color {
  match utils::getTeam(val) {
    Blank => RGB(0,0,0),
    Blue  => RGB(0, 0, val*25),
    Green => {
      let v = val - 10;
      RGB(0, v*25, 0)
    },
    Red   => {
      let v = val - 20;
      RGB(v*25, 0, 0)
    },
    Yellow => {
      let v = val - 30;
      RGB(v*25, v*25, 0)
    }
  }
}

