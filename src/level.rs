use macros::level;

pub type Position = (u8, u8);

#[repr(u8)]
pub enum LevelObject {
  Wall,
  Goal,
}

pub struct Level<const W: usize, const H: usize, const G: usize> {
  layout: [[LevelObject; W]; H],
  boxes: [Position; G],
  player: Position,
}

impl<const W: usize, const H: usize, const G: usize> Level<W, H, G> {
  /// Create a level from bare positions, can create invalid levels, so prefer the level macro
  pub fn new(layout: [[LevelObject; W]; H], boxes: [Position; G], player: Position) -> Self {
    Self {
      layout,
      boxes,
      player,
    }
  }
}

fn test() {
  level!(4);
}

#[repr(u64)]
pub enum Tile {
  Empty = ' ' as u64,
  Box = '📦' as u64,
  Goal = '🎄' as u64,
  BoxOnGoal = '🎁' as u64,
  Player = '🎅' as u64,
}

pub struct LevelView<const W: usize, const H: usize> {
  layout: [[Tile; W]; H],
}
