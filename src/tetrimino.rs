// Can think of tetriminos as existing in a 4x4 grid
type TetriminoGrid = [bool; 16];

pub enum Tetrimino {
  O, I, T, L, J, S, Z
}

impl Tetrimino {     
  // Not sure how to do a roatation without a specific coordinate as center, so
  // treating the array as non-zero units
  const COORDS: [i8; 4] = [-2, -1, 1, 2];

  const BASE_O: TetriminoGrid = [
    true, true, false, false,
    true, true, false, false,
    false, false, false, false,
    false, false, false, false,
  ];
      
  const BASE_I: TetriminoGrid = [
    true, true, true, true,
    false, false, false, false,
    false, false, false, false,
    false, false, false, false,
  ];
     
  const BASE_T: TetriminoGrid = [
    true, true, true, false,
    false, true, false, false,
    false, false, false, false,
    false, false, false, false,
  ];

  const BASE_L: TetriminoGrid = [
    true, true, false, false,
    true, false, false, false,
    true, false, false, false,
    false, false, false, false,
  ];

  const BASE_J: TetriminoGrid = [
    true, true, false, false,
    false, true, false, false,
    false, true, false, false,
    false, false, false, false,
  ];
  
  const BASE_S: TetriminoGrid = [
    true, true, false, false,
    false, true, true, false,
    false, false, false, false,
    false, false, false, false,
  ];

  const BASE_Z: TetriminoGrid = [
    false, true, true, false,
    true, true, false, false,
    false, false, false, false,
    false, false, false, false,
  ];

  fn coord_to_xy_index(coord: i8) -> Option<usize> {
    match coord {
      -2 => Some(0),
      -1 => Some(1),
      1 => Some(2),
      2 => Some(3),
      _ => None
    }
  }
  
  fn flat_index(x: usize, y: usize) -> usize {
    x + (y * 4)
  }
  
  fn rotate_grid(orig_grid: &TetriminoGrid) -> TetriminoGrid {
    let mut new_grid = [false; 16];
    for x in  0..=3 {
      for y in 0..=3 {
        let val = orig_grid[Self::flat_index(x, y)];
        // Array is initialized with false, so no need to waste time unless val is true
        if val {
          // Matrix mult of 
          // (0, -1) * (x, y)
          // (1,0) 
          let new_x = Self::coord_to_xy_index(Self::COORDS[y] * -1).unwrap();
          let new_y = Self::coord_to_xy_index(Self::COORDS[x]).unwrap();
          new_grid[Self::flat_index(new_x, new_y)] = val;
        }
      }
    }
    new_grid
  }
  
  fn rotate_grid_times(orig_grid: TetriminoGrid, times: u8) -> TetriminoGrid {
    if times <= 0 {
      return orig_grid;
    }
    Self::rotate_grid_times(Self::rotate_grid(&orig_grid), times - 1)
  }

  fn get_base_grid(&self) -> TetriminoGrid {
    match self {
      Tetrimino::O => Self::BASE_O,
      Tetrimino::I => Self::BASE_I,
      Tetrimino::T => Self::BASE_T,
      Tetrimino::L => Self::BASE_L,
      Tetrimino::J => Self::BASE_J,
      Tetrimino::S => Self::BASE_S,
      Tetrimino::Z => Self::BASE_Z,
    }
  }

  pub fn get_rotated_grid(&self, times_rotated: u8) -> TetriminoGrid {
    let base_grid = Self::get_base_grid(self);
    Self::rotate_grid_times(base_grid, times_rotated % 4)
  }
}

pub fn print_tetrimino(t: &Tetrimino, rotations: u8) {
  let tetrimino_grid = t.get_rotated_grid(rotations);
  tetrimino_grid.iter().map(|c|
    if *c { "[]" }
    else  { "  " }
  ).enumerate().for_each(|(i, col)| {
    print!("{}", col);
    if ((i+1) % 4) == 0 {
      println!();
    }
  });
}

struct _TetriminoInstance {
  pos : u8,
  pos_y: u8,
  piece: Tetrimino,
  rotations: u8
}

