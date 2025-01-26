use macros::level;

use crate::io::{self, write, writeln};

pub type Position = (usize, usize);

#[derive(Clone, Copy)]
pub enum LevelObject {
  Space,
  Wall,
  Goal,
}

#[derive(Clone)]
pub struct Level<const W: usize, const H: usize, const B: usize> {
  layout: [[LevelObject; W]; H],
  boxes: [Position; B],
  player: Position,
}

impl<const W: usize, const H: usize, const B: usize> Level<W, H, B> {
  /// Create a level from bare positions, can create invalid levels, so prefer the level macro
  pub const fn __new_from_raw(
    layout: [[LevelObject; W]; H],
    boxes: [Position; B],
    player: Position,
  ) -> Self {
    Self {
      layout,
      boxes,
      player,
    }
  }
}

enum Color {
  Default,
  Goal,
}

impl Color {
  fn bg(&self) -> &[u8] {
    match self {
      Self::Default => "\x1b[49m".as_bytes(),
      Self::Goal => "\x1b[42m".as_bytes(),
    }
  }
}

#[derive(Clone, Copy)]
enum Tile {
  Space,
  Box,
  Goal,
  BoxOnGoal,
  Player,
  PlayerOnGoal,
  Wall,
}

impl Tile {
  fn line_one(&self) -> &[u8] {
    match self {
      Self::Wall => "██".as_bytes(),
      Self::Space | Self::Goal => "  ".as_bytes(),
      Self::Box | Self::BoxOnGoal => "▗▖".as_bytes(),
      Self::Player | Self::PlayerOnGoal => "..".as_bytes(),
    }
  }

  fn line_two(&self) -> &[u8] {
    match self {
      Self::Wall => "██".as_bytes(),
      Self::Space | Self::Goal => "  ".as_bytes(),
      Self::Box | Self::BoxOnGoal => "▝▘".as_bytes(),
      Self::Player | Self::PlayerOnGoal => "╰╯".as_bytes(),
    }
  }

  fn color(&self) -> Color {
    match self {
      Self::Goal | Self::BoxOnGoal | Self::PlayerOnGoal => Color::Goal,
      _ => Color::Default,
    }
  }
}

#[derive(Clone)]
pub struct LevelView<const W: usize, const H: usize> {
  layout: [[Tile; W]; H],
}

impl<'a, const W: usize, const H: usize, const B: usize> From<&'a Level<W, H, B>>
  for LevelView<W, H>
{
  fn from(level: &'a Level<W, H, B>) -> Self {
    let mut layout: [[Tile; W]; H] = [[Tile::Space; W]; H];

    for (y, row) in level.layout.into_iter().enumerate() {
      for (x, tile) in row.into_iter().enumerate() {
        layout[y][x] = match tile {
          LevelObject::Space => Tile::Space,
          LevelObject::Goal => Tile::Goal,
          LevelObject::Wall => Tile::Wall,
        };
      }
    }

    for (x, y) in level.boxes {
      layout[y][x] = match layout[y][x] {
        Tile::Goal => Tile::BoxOnGoal,
        _ => Tile::Box,
      }
    }

    let (x, y) = level.player;
    layout[y][x] = Tile::Player;

    Self { layout }
  }
}

impl<const W: usize, const H: usize> LevelView<W, H> {
  pub fn write(&self, fd: u32) {
    for row in self.layout {
      for tile in row {
        write!(fd, tile.color().bg(), tile.line_one());
      }

      writeln!(fd, &[]);

      for tile in row {
        write!(fd, tile.color().bg(), tile.line_two());
      }

      writeln!(fd, &[]);
    }
  }

  pub fn print(&self) {
    self.write(io::STDOUT_FILENO);
  }
}

level!(LEVEL_0 = ["#####", "#pbg#", "#####"]);
