use macros::level;

pub type Position = (usize, usize);

pub enum Movement {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Clone)]
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

  fn bounded_move((x, y): Position, movement: &Movement) -> Option<Position> {
    match movement {
      Movement::Up if y > 0 => Some((x, y - 1)),
      Movement::Down if y < H - 1 => Some((x, y + 1)),
      Movement::Left if x > 0 => Some((x - 1, y)),
      Movement::Right if x < W - 1 => Some((x + 1, y)),
      _ => None,
    }
  }

  fn is_position_solid(&self, (x, y): &Position) -> bool {
    matches!(self.layout[*y][*x], LevelObject::Wall)
  }

  fn get_box_at_position(&self, position: &Position) -> Option<usize> {
    for (index, box_pos) in self.boxes.into_iter().enumerate() {
      if box_pos == *position {
        return Some(index);
      }
    }

    None
  }

  pub fn move_player(&mut self, movement: &Movement) {
    let Some(position) = Self::bounded_move(self.player, movement) else {
      return;
    };

    if self.is_position_solid(&position) {
      return;
    };

    if let Some(box_index) = self.get_box_at_position(&position) {
      let Some(box_position) = Self::bounded_move(position, movement) else {
        return;
      };

      if self.is_position_solid(&box_position) || self.get_box_at_position(&box_position).is_some()
      {
        return;
      };

      self.boxes[box_index] = box_position;
    }

    self.player = position;
  }

  pub fn is_solved(&self) -> bool {
    for (y, row) in self.layout.iter().enumerate() {
      'positions: for (x, tile) in row.into_iter().enumerate() {
        if matches!(tile, LevelObject::Goal) {
          for box_pos in self.boxes {
            if box_pos == (x, y) {
              continue 'positions;
            }
          }

          return false;
        }
      }
    }

    true
  }
}

//level!(level_1 = ["p b g"]);

//level!(level_2 = ["p bg", "gb  "]);

//level!(level_3 = ["   g", "P#b ", "   b", "    "]);

level!(level_4 = ["p ####", " bb gg", "      ", "      "]);
