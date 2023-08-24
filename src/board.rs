#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Board {
    // indexed by col, row
    stones_all: u64,
    stones_player: u64,
    num_moves: u8,
}

impl Board {
    pub const WIDTH: u8 = 7;
    pub const HEIGHT: u8 = 6;
    pub const MAX_SCORE: i32 = ((Board::WIDTH * Board::HEIGHT + 1) as i32) / 2 - 3;
    pub const MIN_SCORE: i32 = -((Board::WIDTH * Board::HEIGHT) as i32) / 2 + 3;
    pub const exploration_order: [u8; 7] = [3, 2, 4, 1, 5, 0, 6];

    pub fn new() -> Board {
        Board {
            stones_all: 0,
            stones_player: 0,
            num_moves: 0,
        }
    }

    pub fn construct(instructions: &str) -> Board {
        let mut board = Board::new();
        for play in instructions.chars() {
            if !play.is_digit(10) {
                continue;
            }
            let col = play.to_digit(10).unwrap() as u8;
            board.play_col(col - 1);
        }
        board
    }

    pub fn hash(&self) -> u64 {
        self.stones_all + self.stones_player
    }

    fn bottom_mask() -> u64 {
        // why can't I use addition here?
        (0..Board::WIDTH).fold(0_u64, |accum, col| {
            accum + (1_u64 << col * (Board::HEIGHT + 1))
        })
    }

    fn board_mask() -> u64 {
        ((1_u64 << Board::HEIGHT) - 1) * Board::bottom_mask()
    }

    fn can_play(&self, col: u8) -> bool {
        self.stones_all & Board::top_mask(col) == 0
    }

    fn play(&mut self, move_mask: u64) {
        self.stones_player ^= self.stones_all;
        self.stones_all |= move_mask;
        self.num_moves += 1;
    }

    pub fn play_col(&mut self, col: u8) {
        self.play((self.stones_all + Board::bottom_mask_col(col)) & Board::col_mask(col));
    }

    pub fn can_win_next(&self) -> bool {
        self.winning_moves() & self.possible() != 0
    }

    fn possible(&self) -> u64 {
        (self.stones_all + Board::bottom_mask()) & Board::board_mask()
    }

    pub fn winning_move(&self) -> u8 {
        let possible = self.possible();
        let winning = self.winning_moves();
        for i in 0..Board::WIDTH {
            if Board::col_mask(i) & winning & possible != 0 {
                return i;
            }
        }
        return 255; // this should never happen
    }

    pub fn possible_move(&self) -> u8 {
        let possible = self.possible();
        for i in 0..Board::WIDTH {
            if Board::col_mask(i) & possible != 0 {
                return i;
            }
        }
        return 0; // this should never happen
    }

    pub fn action_score(&self, action: u64) -> i32 {
        let winning_moves =
            Board::compute_winning_moves(self.stones_player | action, self.stones_all);
        Board::pop_count(winning_moves)
    }

    fn pop_count(mut value: u64) -> i32 {
        let mut count = 0;
        while value > 0 {
            count += 1;
            value = value & (value - 1);
        }
        count
    }

    fn opponent_winning_moves(&self) -> u64 {
        let result =
            Board::compute_winning_moves(self.stones_player ^ self.stones_all, self.stones_all);
        result
    }

    pub fn nonlosing_moves(&self) -> u64 {
        let mut possible = self.possible();
        let opponent_win = self.opponent_winning_moves();
        let forced_moves = possible & opponent_win;
        //(possible & opponent_win) & !(opponent_win >> 1)
        if forced_moves > 0 {
            if (forced_moves & (forced_moves - 1)) == 0 {
                possible = forced_moves;
            } else {
                return 0;
            }
        }
        possible & !(opponent_win >> 1)
    }

    pub fn winning_moves(&self) -> u64 {
        let mut result = 0_u64;
        let mut intermediary;
        // vertical
        result |= self.stones_player << 1 & self.stones_player << 2 & self.stones_player << 3;

        //horizontal
        intermediary = self.stones_player << (Board::HEIGHT + 1)
            & self.stones_player << (Board::HEIGHT + 1) * 2;
        result |= intermediary & self.stones_player << (Board::HEIGHT + 1) * 3;
        result |= intermediary & self.stones_player >> (Board::HEIGHT + 1);
        intermediary = self.stones_player >> (Board::HEIGHT + 1)
            & self.stones_player >> (Board::HEIGHT + 1) * 2;
        result |= intermediary & self.stones_player >> (Board::HEIGHT + 1) * 3;
        result |= intermediary & self.stones_player << (Board::HEIGHT + 1);
        // diagonal 1
        intermediary =
            self.stones_player << Board::HEIGHT & self.stones_player << 2 * Board::HEIGHT;
        result |= intermediary & self.stones_player >> Board::HEIGHT;
        result |= intermediary & self.stones_player << 3 * Board::HEIGHT;
        intermediary =
            self.stones_player >> Board::HEIGHT & self.stones_player >> 2 * Board::HEIGHT;
        result |= intermediary & self.stones_player << Board::HEIGHT;
        result |= intermediary & self.stones_player >> 3 * Board::HEIGHT;

        // diagonal 2
        intermediary =
            self.stones_player << Board::HEIGHT + 2 & self.stones_player << 2 * (Board::HEIGHT + 2);
        result |= intermediary & self.stones_player << (Board::HEIGHT + 2) * 3;
        result |= intermediary & self.stones_player >> Board::HEIGHT + 2;
        intermediary =
            self.stones_player >> Board::HEIGHT + 2 & self.stones_player >> 2 * (Board::HEIGHT + 2);
        result |= intermediary & self.stones_player >> (Board::HEIGHT + 2) * 3;
        result |= intermediary & self.stones_player << Board::HEIGHT + 2;

        result & (Board::board_mask() ^ self.stones_all)
        //result
    }

    pub fn compute_winning_moves(stones_player: u64, stones_all: u64) -> u64 {
        let mut result = 0_u64;
        let mut intermediary;
        // vertical
        result |= stones_player << 1 & stones_player << 2 & stones_player << 3;

        //horizontal
        intermediary =
            stones_player << (Board::HEIGHT + 1) & stones_player << (Board::HEIGHT + 1) * 2;
        result |= intermediary & stones_player << (Board::HEIGHT + 1) * 3;
        result |= intermediary & stones_player >> (Board::HEIGHT + 1);
        intermediary =
            stones_player >> (Board::HEIGHT + 1) & stones_player >> (Board::HEIGHT + 1) * 2;
        result |= intermediary & stones_player >> (Board::HEIGHT + 1) * 3;
        result |= intermediary & stones_player << (Board::HEIGHT + 1);
        // diagonal 1
        intermediary = stones_player << Board::HEIGHT & stones_player << 2 * Board::HEIGHT;
        result |= intermediary & stones_player >> Board::HEIGHT;
        result |= intermediary & stones_player << 3 * Board::HEIGHT;
        intermediary = stones_player >> Board::HEIGHT & stones_player >> 2 * Board::HEIGHT;
        result |= intermediary & stones_player << Board::HEIGHT;
        result |= intermediary & stones_player >> 3 * Board::HEIGHT;

        // diagonal 2
        intermediary =
            stones_player << Board::HEIGHT + 2 & stones_player << 2 * (Board::HEIGHT + 2);
        result |= intermediary & stones_player << (Board::HEIGHT + 2) * 3;
        result |= intermediary & stones_player >> Board::HEIGHT + 2;
        intermediary =
            stones_player >> Board::HEIGHT + 2 & stones_player >> 2 * (Board::HEIGHT + 2);
        result |= intermediary & stones_player >> (Board::HEIGHT + 2) * 3;
        result |= intermediary & stones_player << Board::HEIGHT + 2;

        result & (Board::board_mask() ^ stones_all)
        //result
    }

    pub fn nb_moves(&self) -> u8 {
        self.num_moves
    }

    fn top_mask(col: u8) -> u64 {
        1_u64 << (Board::HEIGHT - 1) << (col * (Board::HEIGHT + 1))
    }

    pub fn col_mask(col: u8) -> u64 {
        (1_u64 << Board::HEIGHT) - 1 << (col * (Board::HEIGHT + 1))
    }

    fn bottom_mask_col(col: u8) -> u64 {
        1_u64 << (col * (Board::HEIGHT + 1))
    }

    fn copy(&self) -> Board {
        Board {
            stones_all: self.stones_all,
            stones_player: self.stones_player,
            num_moves: self.num_moves,
        }
    }

    fn accessor(target: u64, row: u8, col: u8) -> bool {
        let index = (col * (Board::HEIGHT + 1) + row) as u64;
        (target & (1_u64 << index)) > 0
    }

    pub fn display(&self) {
        println!("-------");
        for row in (0..Board::HEIGHT).rev() {
            for col in 0..Board::WIDTH {
                if Board::accessor(self.stones_all, row, col) {
                    if Board::accessor(self.stones_player, row, col) {
                        print!("X");
                    } else {
                        print!("O");
                    }
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
        println!("-------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_board() {
        let board = Board::new();
        assert_eq!(board.stones_all, 0);
        assert_eq!(board.stones_player, 0);
    }

    #[test]
    fn playing_col_simple() {
        let mut board = Board::new();
        let mut total = 0;
        assert!(board.can_play(0));
        for i in 0..Board::HEIGHT {
            total += 2_u64.pow(i as u32);
            board.play_col(0);
            assert_eq!(board.stones_all, total)
        }
        assert!(!board.can_play(0));
    }

    #[test]
    fn playing_col_medium() {
        let mut board = Board::new();
        board.play_col(1);
        assert_eq!(board.stones_all, 2_u64.pow(Board::HEIGHT as u32 + 1));
        board.play_col(0);
        assert_eq!(board.stones_all, 2_u64.pow(Board::HEIGHT as u32 + 1) + 1);
    }

    #[test]
    fn accessor() {
        let mut board = Board::new();
        assert!(!Board::accessor(board.stones_all, 0, 0));
        assert!(!Board::accessor(board.stones_all, 1, 0));

        board.play_col(0);
        assert!(Board::accessor(board.stones_all, 0, 0));
        assert!(!Board::accessor(board.stones_all, 1, 0));

        board.play_col(0);
        assert!(Board::accessor(board.stones_all, 0, 0));
        assert!(Board::accessor(board.stones_all, 1, 0));
    }

    #[test]
    fn winning() {
        let board = Board::construct("131415");
        assert_eq!(board.winning_moves(), 1_u64 << 3);

        let board = Board::construct("162636");
        assert!(board.can_win_next());

        let board = Board::construct("472737");
        assert!(board.can_win_next());
        assert!(board.can_win_next());

        let board = Board::construct("1223344445");
        assert!(board.can_win_next());

        let board = Board::construct("525354");
        assert!(board.can_win_next());
    }

    #[test]
    fn opponent_win() {
        let board_org = Board::construct("13141");
        let mut board = Board::construct("13141");
        let op_win = board.opponent_winning_moves();
        assert_eq!(board, board_org);

        assert_eq!(op_win, 1_u64 << 3);
        let mut board = Board::construct("1");

        assert_eq!(
            board.nonlosing_moves() ^ (1_u64 << 1) | 1_u64,
            Board::bottom_mask()
        );
    }
}
