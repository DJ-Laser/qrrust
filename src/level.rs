use macros::level;

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

  pub fn layout(&self) -> &[[LevelObject; W]; H] {
    &self.layout
  }

  pub fn boxes(&self) -> &[Position; B] {
    &self.boxes
  }

  pub fn player(&self) -> &Position {
    &self.player
  }
}

level!(LEVEL_0 = ["P b g", " ###b", "     "]);
