
use colors;
use patterns;
use utils;
use sdl;

pub struct Config {
  going: bool,
  width: uint,
  height: uint,
  theme: colors::Theme,
  pattern: patterns::Pattern,
  team: utils::Team
}

impl Config {
  pub fn initScreen(&self) -> ~sdl::video::Surface {
    match sdl::video::set_video_mode(
            self.width as int,
            self.height as int,
            32,
            [sdl::video::HWSurface],
            [sdl::video::DoubleBuf]) {
      Ok(screen) => screen,
      Err(err) => fail!("failed to set video mode: {}", err)
    }
  }
}

