pub type Position = (u8, u8);

#[repr(u8)]
pub enum LevelObject {
  Space,
  Wall,
  Goal,
}

pub struct Level<const W: usize, const H: usize, const B: usize> {
  layout: [[LevelObject; W]; H],
  boxes: [Position; B],
  player: Position,
}

impl<const W: usize, const H: usize, const B: usize> Level<W, H, B> {
  /// Create a level from bare positions, can create invalid levels, so prefer the level macro
  pub fn __new_from_raw(
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
