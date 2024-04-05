pub struct Board {
    grid: Vec<i32>
}

impl Default for Board {
    fn default() -> Self {
        let mut new_board = Self { grid: vec![0; 9] };
        new_board.reset_board();
        new_board
    }
}

const EMPTY_VALUE: i32 = 0; 
impl Board {
    pub fn reset_board(&mut self) {
        self.grid.fill(EMPTY_VALUE);
    }

    pub fn get_play(&self, cell_id: u8) -> i32 {
        self.grid[cell_id as usize]
    }

    pub fn set_play(&mut self, cell_id: u8, play_value: i32) {
        self.grid[cell_id as usize] = play_value;
    }

    pub fn is_cell_empty(&self, cell_id: u8) -> bool {
        self.grid[cell_id as usize] == EMPTY_VALUE
    }
}