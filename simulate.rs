extern crate sdl;

// use std::rand::Rng;
// use std::rand;

use sdl::video::{Color, RGB, Surface};

struct Matrix {
  values: ~[~[u8]],
  width: uint,
  height: uint
}

struct Config {
  width: uint,
  height: uint
}

enum Team {
  Blank = 0,
  Blue = 1,
  Green = 2,
  Red = 3
}

fn numTeam(val: u8) -> Team {
  match val {
    0 => Blank,
    1 => Blue,
    2 => Green,
    3 => Red,
    _ => Blank
  }
}

fn maketrix(width: uint, height: uint) -> ~[~[u8]] {
  let mut matrix: ~[~[u8]] = ~[];
  for _ in range(0, height) {
    let mut sub: ~[u8] = ~[];
    for _ in range(0, width) {
      sub.push(0);
    }
    matrix.push(sub)
  }
  matrix
}

fn init(width: uint, height: uint) -> (Matrix, Matrix) {
  let one = Matrix {
    values: maketrix(width, height),
    width: width,
    height: height
  };
  let two = Matrix {
    values: maketrix(width, height),
    width: width,
    height: height
  };
  // x[0][0] = 100;
  (one, two)
}

fn fill(mx: &mut Matrix, x: int, y: int, w: int, h: int, val: u8) {
  for cx in range(x, x+w) {
    for cy in range(y, y+h) {
      mx.values[cy][cx] = val;
    }
  }
}

fn getRich(val: u8) -> (Team, u8) {
  match val {
     0     => (Blank, 0),
     1..10 => (Blue, val),
    11..20 => (Green, val - 10),
    12..30 => (Red, val - 20),
    _      => (Blank, 0)
  }
}

fn getPoor(team: Team, val: u8) -> u8 {
  match team {
    Blank => 0,
    Blue => val,
    Green => val + 10,
    Red => val + 20
  }
}

fn getTeam(val: u8) -> Team {
  match val {
     0     => Blank,
     1..10 => Blue,
    11..20 => Green,
    12..30 => Red,
    _      => Blank
  }
}

fn light(val: u8) -> Color {
  match getTeam(val) {
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
  match getTeam(val) {
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

fn colorize(val: u8) -> Color {
  if (false) {
    light(val)
  } else {
    dark(val)
  }
}

fn draw(config: &Config, screen: &sdl::video::Surface, mx: &Matrix) {
  let xscale = config.width as i16 / mx.width as i16;
  let yscale = config.height as i16/ mx.height as i16;
  for y in range(0u, mx.height) {
    for x in range(0u, mx.width) {
      screen.fill_rect(Some(sdl::Rect {
        x: (x as i16) * xscale,
        y: (y as i16) * yscale,
        w: xscale as u16,
        h: yscale as u16
      }), colorize(mx.values[y][x]));
    }
  }
}

fn initScreen(config: Config) -> ~Surface {
  match sdl::video::set_video_mode(
          config.width as int,
          config.height as int,
          32,
          [sdl::video::HWSurface],
          [sdl::video::DoubleBuf]) {
    Ok(screen) => screen,
    Err(err) => fail!("failed to set video mode: {}", err)
  }
}

fn upOne(old: &Matrix, current: &mut Matrix, x: uint, y: uint) {
  let moves = [(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1), (0, -1)];
  let mut counts:[u8, ..4] = [0,0,0,0];
  let (team, cval) = getRich(old.values[y][x]);
  for i in range(0, 8) {
    let (dx, dy) = moves[i];
    if dx + x < 0 ||
        dy + y < 0 ||
        dx + x >= old.width ||
        dy + y >= old.height {
      continue;
    }
    let strength = match dx + dy {
      1 | -1 => 2, // straight
      _      => 1  // diagonal
    };
    let (oteam, oval) = getRich(old.values[dy+y][dx+x]);
    counts[oteam as int] += strength * oval;
  }
  match team {
    Blank => upBlank(current, x, y, &counts),
    _ => upTeam(current, x, y, teamDiff(team, &counts), team, cval)
  }
}

fn upBlank(current: &mut Matrix, x: uint, y: uint, counts: &[u8, ..4]) {
  let mut which: u8 = 0;
  let mut what: u8 = 0;
  for i in range(0 as u8, 4) {
    if counts[i] > what {
      what = counts[i];
      which = i;
    }
  }
  current.values[y][x] = getPoor(numTeam(which), what);
}

fn predator(team: Team) -> Team {
  match team {
    Blank => Blank,
    Green => Blue,
    Blue => Red,
    Red => Green
  }
}

fn prey(team: Team) -> Team {
  match team {
    Blank => Blank,
    Green => Red,
    Blue => Green,
    Red => Blue
  }
}

fn upTeam(current: &mut Matrix, x: uint, y: uint, diff: i8, team: Team, cval: u8) {
  let now = cval as i8 + diff;
  current.values[y][x] = if now < 0 {
    0
  } else if now > 10 {
    getPoor(team, 10)
  } else {
    getPoor(team, now as u8)
  };
}

fn teamDiff(team: Team, counts: &[u8, ..4]) -> i8 {
  // let empty = counts[Blank as int] as i8;
  let food = counts[prey(team) as int] as i8;
  let danger = counts[predator(team) as int] as i8;
  let friends = counts[team as int] as i8;
  // 12
  if danger > friends {
    -1// -danger + friends + food/2
  } else if friends > 6 {
    -1
  } else if food > 0 {
    1
  } else if friends < 3 {
    1
  } else {
    0
  }
}

fn advance(old: &Matrix, current: &mut Matrix) {
  for x in range(0u, old.width) {
    for y in range(0u, old.height) {
      upOne(old, current, x, y);
    }
  }
}

#[main]
pub fn main() {
  sdl::init([sdl::InitVideo]);
  sdl::wm::set_caption("Rust Simulator", "rust-sdl");

  let config = Config {width: 400, height: 400};

  // let mut rng = rand::rng();
  let screen = initScreen(config);

  let (mut one, mut two) = init(100, 100);
  let mut old = &mut one;
  let mut current = &mut two;
  let mut third:&mut Matrix;

  fill(current, 30, 60, 10, 10, 1);
  for i in range(0, 30) {
    fill(current, 0, 10 + i * 2, 100, 2, i as u8);
    //fill(current, 0, 0 + i*4, 20, 4, i as u8);
  }

  let going = true;

  'main : loop {
    'event : loop {
      match sdl::event::poll_event() {
        sdl::event::QuitEvent => break 'main,
        sdl::event::NoEvent => break 'event,
        sdl::event::KeyEvent(k, _, _, _)
                  if k == sdl::event::EscapeKey
                      => break 'main,
                _ => {}
      }
    }
    if going {
      third = old;
      old = current;
      current = third;
      advance(old, current);
    }
    draw(&config, screen, current);

    screen.flip();
  }

  sdl::quit();
}

