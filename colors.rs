use sdl::video::{Color, RGB, Surface};
use utils;
use utils::{Blank, Blue, Green, Red, Yellow};

pub enum Theme {
  Light,
  Dark,
  Orange
}

pub fn colorize(theme: Theme, val: u8) -> Color {
  match theme {
    Light => light(val),
    Dark  => dark(val),
    Orange => orange(val)
  }
}

pub fn nextTheme(theme: Theme) -> Theme {
  match theme {
    Light => Dark,
    Dark => Orange,
    Orange => Light
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
      RGB(v*10, 0, v*13)
    },
    Yellow => {
      let v = val - 30;
      RGB(v*25, v*25, 0)
    }
  }
}

fn orange(val: u8) -> Color {
  match utils::getTeam(val) {
    Blank => RGB(0,0,0),
    Blue  => RGB(val*15, val*12, 0),
    Green => {
      let v = val - 10;
      RGB(v*25, v*9, 0)
    },
    Red   => {
      let v = val - 20;
      RGB(v*15, v*2, 0)
    },
    Yellow => {
      let v = val - 30;
      RGB(v*25, v*25, 0)
    }
  }
}

