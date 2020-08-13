#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Board {
    // indexed by col, row
    stones_all: u64,
    stones_player: u64,
    num_moves: u8,
}

impl Board {
    pub const WIDTH: u8 = 7;
    pub const HEIGHT: u8 = 6;

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
        self.stones_all + Board::bottom_mask() & Board::board_mask()
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

        result
    }

    fn nb_moves(&self) -> u8 {
        self.num_moves
    }

    fn top_mask(col: u8) -> u64 {
        1_u64 << (Board::HEIGHT - 1) << (col * (Board::HEIGHT + 1))
    }

    fn col_mask(col: u8) -> u64 {
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

pub fn negamax(position: &Board) -> i32 {
    if position.nb_moves() == Board::HEIGHT * Board::WIDTH {
        return 0;
    }
    if position.can_win_next() {
        let mut score = Board::HEIGHT * Board::WIDTH + 1 - position.nb_moves();
        score /= 2; // allows encoding for different players is symmetric
        return score as i32;
    }

    let mut best_score = (Board::WIDTH * Board::WIDTH) as i32 * -1;
    for x in 0..Board::WIDTH {
        if position.can_play(x) {
            let mut next_position = position.copy();
            next_position.play_col(x);
            let score = -negamax(&next_position);
            if score > best_score {
                best_score = score;
            }
        }
    }

    best_score
}

#[cfg(test)]
mod board_tests {
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
}
