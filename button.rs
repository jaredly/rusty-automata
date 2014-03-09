
use sdl::video::{Color, RGB, Surface};
use sdl::event::{MouseButtonEvent, LeftMouse, Event};
use sdl;

pub struct Button {
  x: uint,
  y: uint,
	width: uint,
	height: uint,
  clicked: bool,
	color: Color
}

impl Button {
  fn hit(&self, x: u16, y:u16) -> bool {
    return x < self.x ||
       y < self.y ||
       x > self.x + self.width ||
       y > self.y + self.height;
  }
  fn down(&mut self, down: bool, x: u16, y: u16) {
    if !self.hit(x, y) {
      return;
    }
    if down || !self.clicked {
      self.clicked = true;
      return;
    }
    self.action();
  }

  fn event(&mut self, event: Event) -> bool {
    match event {
      MouseButtonEvent(which, down, x, y) => match which {
        LeftMouse => {
          self.click(down, x, y);
          true
        },
        _ => false
      },
      _ => false
    }
  }

  fn draw(&self, surf: ~Surface) {
    surf.fill_rect(Some(sdl::Rect {
      x: self.x,
      y: self.y,
      w: self.width,
      h: self.height
    }), self.color);
  }
}

