use ratatui::style::Color;

use crate::board::Board;

pub struct App {
    board: Board,
    players: [Player;2],
    pub selected_cell_id: u8,
    current_player_index: usize,
    winner_index: Option<usize>,
    missing_free_cells: u8,
    pub game_over: bool,
    pub should_quit: bool
}
#[allow(dead_code)]
pub enum Action {
    Quit,
    Render,
    None,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Debug)]
pub struct Player {
    pub play_symbol: &'static str,
    pub symbol_color: Color,
}

impl App {
    pub fn new() -> Self {
        let players = [Player{ play_symbol: "x", symbol_color: Color::Green }, 
                                    Player{ play_symbol: "o", symbol_color: Color::Red }];

        Self { 
            board: Board::default(), 
            players: players,
            selected_cell_id: 4,
            current_player_index: 0,
            winner_index: None,
            game_over: false, 
            missing_free_cells: 9,
            should_quit: false, 
        }
    }

    pub fn restart(&mut self) {
        self.board.reset_board();
        self.selected_cell_id = 4;
        self.current_player_index = 0;
        self.winner_index = None;
        self.game_over = false;
        self.missing_free_cells = 9;
    }

    // Move selection index assuming grid is 3x3
    pub fn move_selection(&mut self, dir: Direction) {
        match dir {
            Direction::Up => if self.selected_cell_id > 2 { self.selected_cell_id -= 3; },
            Direction::Down => if self.selected_cell_id < 6 { self.selected_cell_id += 3; },
            Direction::Left => if self.selected_cell_id % 3 > 0 { self.selected_cell_id -= 1; },
            Direction::Right => if self.selected_cell_id % 3 < 2 { self.selected_cell_id += 1; },
        }
    }

    // pub fn play_on_selection<'b>(&'b mut self) where 'b:'a {
    pub fn play_on_selection(&mut self) {
        if !self.board.is_cell_empty(self.selected_cell_id) {
            return;
        }

        let play_value = match self.current_player_index {
            0 => 1,
            1 => -1,
            _ => panic!("invalid player index")
        };
        
        self.board.set_play(self.selected_cell_id, play_value);
        self.missing_free_cells -= 1;

        self.winner_index = self.check_for_winner();
        if self.winner_index.is_some() {
            self.game_over = true;
            return;
        }

        // There are no more empty cells. Game over with a tie!
        if self.missing_free_cells == 0 {
            self.game_over = true;
            return;
        }
        
        self.current_player_index = 1 - self.current_player_index;
    }

    const EMPTY_SYMBOL: &str = " "; 
    pub fn get_play(&self, cell_id: u8) -> (&str, Color) {
        if self.board.is_cell_empty(cell_id) {
            return (Self::EMPTY_SYMBOL, Color::Reset);
        }

        let play = self.board.get_play(cell_id);
        let player = match play {
            1 => &self.players[0],
            -1 => &self.players[1],
            _ => panic!("invalid play value '{play}' at cell '{cell_id}'")
        };

        (player.play_symbol, player.symbol_color)
    }

    pub fn get_winner(&self) -> Option<&Player> {
        if self.winner_index.is_some() {
            Some(&self.players[self.winner_index.unwrap()])
        } else {
            None
        }
    }

    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player_index]
    }
    
    // Returns the index of the winning player
    fn check_for_winner(&self) -> Option<usize> {
        let mut possible_winner = [0;8];
        // Check for winner in rows
        for i in 0..3 {
            for j in 0..3 {
                possible_winner[i] += self.board.get_play((i*3 + j).try_into().unwrap());
            }
        }
        // Check for winner in cols
        for i in 0..3 {
            for j in 0..3 {
                possible_winner[i+3] += self.board.get_play((i + j*3).try_into().unwrap());
            }
        }
        // Check for winner in diagonal
        for j in [0, 4, 8] {
            possible_winner[6] += self.board.get_play(j);
        }
        for j in [2, 4, 6] {
            possible_winner[7] += self.board.get_play(j);
        }

        let winner = possible_winner.iter().find_map(|&sum| {
            if sum == 3 {
                Some(0)
            } else if sum == -3 {
                Some(1)
            } else {
                None
            }
        });

        winner
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_winners() {
        let mut app = App::new();

        app.board.set_play(0, 1);
        app.board.set_play(1, 1);

        assert_eq!(app.check_for_winner(), None);
    }

    #[test]
    fn test_winning_horizontal() {
        let mut app = App::new();

        app.board.set_play(0, 1);
        app.board.set_play(1, 1);
        app.board.set_play(2, 1);

        assert_eq!(app.check_for_winner(), Some(0));
    }

    #[test]
    fn test_winning_vertical() {
        let mut app = App::new();

        app.board.set_play(0, -1);
        app.board.set_play(3, -1);
        app.board.set_play(6, -1);

        assert_eq!(app.check_for_winner(), Some(1));
    }

    #[test]
    fn test_winning_top_left_diagonal() {
        let mut app = App::new();

        app.board.set_play(0, -1);
        app.board.set_play(4, -1);
        app.board.set_play(8, -1);

        assert_eq!(app.check_for_winner(), Some(1));
    }

    #[test]
    fn test_winning_top_right_diagonal() {
        let mut app = App::new();

        app.board.set_play(2, 1);
        app.board.set_play(4, 1);
        app.board.set_play(6, 1);

        assert_eq!(app.check_for_winner(), Some(0));
    }
}