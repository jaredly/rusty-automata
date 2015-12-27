
use rules::RuleKey;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::mouse::Mouse;
use sdl2::event::Event;
use sdl2::rect::Rect;

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
  /*
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
    */
    false
  }

  pub fn draw(&self, surf: &mut Renderer) {
  surf.set_draw_color(self.color);
    surf.draw_rect(Rect::new (
      self.x as i32,
      self.y as i32,
      self.width as u32,
      self.height as u32
    ).unwrap().unwrap());
    surf.set_draw_color(Color::RGB(0,0,0));
    surf.draw_rect(Rect::new(
      (self.x + self.width/2 - 1) as i32,
      self.y as i32,
      2,
      self.height as u32
    ).unwrap().unwrap());
  }
}

