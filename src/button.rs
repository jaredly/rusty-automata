
use rules::RuleKey;
use sdl::video::{Color, RGB, Surface};
use sdl::event::{Event, Mouse};
use sdl;

pub struct Button {
  pub x: usize,
  pub y: usize,
  pub width: usize,
  pub height: usize,
  pub clicked: bool,
  pub color: Color,
  pub value: isize,
  pub action: RuleKey
}

impl Button {
  fn hit(&self, x: usize, y:usize) -> bool {
    return x > self.x &&
           y > self.y &&
           x < self.x + self.width &&
           y < self.y + self.height;
  }

  fn click(&mut self, down: bool, x: u16, y: u16) {
    if down || !self.clicked {
      self.clicked = true;
      return;
    }
    self.value += if (x as usize) < self.x + self.width/2 {
      1
    } else {
      -1
    };
    if self.value < 0 {
      self.value = 0
    }
  }

  pub fn event(&mut self, event: &Event) -> bool {
    match event {
      &Event::MouseButton(which, down, x, y) => match which {
        Mouse::Left => {
          if !self.hit(x as usize, y as usize) {
            return false;
          }
          self.click(down, x, y);
          true
        },
        _ => false
      },
      _ => false
    }
  }

  pub fn draw(&self, surf: &Surface) {
    surf.fill_rect(Some(sdl::Rect {
      x: self.x as i16,
      y: self.y as i16,
      w: self.width as u16,
      h: self.height as u16
    }), self.color);
    surf.fill_rect(Some(sdl::Rect {
      x: (self.x + self.width/2 - 1) as i16,
      y: self.y as i16,
      w: 2,
      h: self.height as u16
    }), RGB(0,0,0));
  }
}

