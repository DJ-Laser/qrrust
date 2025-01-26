use crate::{
  io::{self, write, writeln},
  level::{Level, LevelObject},
};

enum Color {
  Default,
  Goal,
}

impl Color {
  const DEFAULT_BG_CODE: &'static [u8] = "\x1b[49m".as_bytes();
  const GOAL_BG_CODE: &'static [u8] = "\x1b[42m".as_bytes();

  fn bg(&self) -> &[u8] {
    match self {
      Self::Default => Self::DEFAULT_BG_CODE,
      Self::Goal => Self::GOAL_BG_CODE,
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
  const WALL_STR: &'static [u8] = "██".as_bytes();
  const SPACE_STR: &'static [u8] = "  ".as_bytes();

  fn line_one(&self) -> &'static [u8] {
    match self {
      Self::Wall => Self::WALL_STR,
      Self::Space | Self::Goal => Self::SPACE_STR,
      Self::Box | Self::BoxOnGoal => "┏┓".as_bytes(),
      Self::Player | Self::PlayerOnGoal => "..".as_bytes(),
    }
  }

  fn line_two(&self) -> &'static [u8] {
    match self {
      Self::Wall => Self::WALL_STR,
      Self::Space | Self::Goal => Self::SPACE_STR,
      Self::Box | Self::BoxOnGoal => "┗┛".as_bytes(),
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

    for (y, row) in level.layout().into_iter().enumerate() {
      for (x, tile) in row.into_iter().enumerate() {
        layout[y][x] = match tile {
          LevelObject::Space => Tile::Space,
          LevelObject::Goal => Tile::Goal,
          LevelObject::Wall => Tile::Wall,
        };
      }
    }

    for (x, y) in level.boxes() {
      layout[*y][*x] = match layout[*y][*x] {
        Tile::Goal => Tile::BoxOnGoal,
        _ => Tile::Box,
      }
    }

    let (x, y) = level.player();
    layout[*y][*x] = match layout[*y][*x] {
      Tile::Goal => Tile::PlayerOnGoal,
      _ => Tile::Player,
    };

    Self { layout }
  }
}

impl<const W: usize, const H: usize> LevelView<W, H> {
  fn write_vertical_bounds(fd: u32) {
    for _ in 0..(W + 2) {
      write!(fd, Tile::WALL_STR)
    }

    writeln!(fd, &[]);
  }

  fn write_row(fd: u32, row: &[Tile; W], line_type: fn(&Tile) -> &'static [u8]) {
    write!(fd, Tile::WALL_STR);

    for tile in row {
      write!(fd, tile.color().bg(), line_type(tile));
    }

    writeln!(fd, Color::DEFAULT_BG_CODE, Tile::WALL_STR);
  }

  pub fn write(&self, fd: u32) {
    Self::write_vertical_bounds(fd);

    for row in self.layout {
      Self::write_row(fd, &row, Tile::line_one);
      Self::write_row(fd, &row, Tile::line_two);
    }

    Self::write_vertical_bounds(fd);
  }

  pub fn print(&self) {
    self.write(io::STDOUT_FILENO);
  }
}
