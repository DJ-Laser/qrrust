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

#[derive(Clone, Copy)]
pub enum Tile {
  Space,
  Box,
  Goal,
  BoxOnGoal,
  Player,
  Wall,
}

impl Tile {
  fn to_bytes(&self) -> &[u8] {
    match self {
      Self::Space => b" ",
      Self::Box => b"\xE2\x98\x90",       // ☐
      Self::Goal => b"x",                 // x
      Self::BoxOnGoal => b"\xE2\x98\x92", // ☒
      Self::Player => b"\xE2\x8F\xA3",    // ⏣ Potential symbols: ⏇  ⍙ ♟
      Self::Wall => b"\xE2\x96\x88",      // █
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
        write!(fd, tile.to_bytes());
      }

      writeln!(fd, &[]);
    }
  }

  pub fn print(&self) {
    self.write(io::STDOUT_FILENO);
  }
}

level!(LEVEL_0 = ["#######", "#p b g#", "#######"]);
