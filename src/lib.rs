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

    fn hash(&self) -> u64 {
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

    fn action_score(&self, action: u64) -> i32 {
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
        let result = Board::compute_winning_moves(self.stones_player ^ self.stones_all, self.stones_all);
        result
    }

    fn nonlosing_moves(&self) -> u64 {
        let mut possible = self.possible();
        let opponent_win = self.opponent_winning_moves();
        let forced_moves = possible & opponent_win;
        //(possible & opponent_win) & !(opponent_win >> 1)
        if forced_moves > 0 {
            if (forced_moves & (forced_moves - 1)) == 0 {
                return 0;
            } else {
                possible = forced_moves;
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

pub fn solve(position: Board, table: &mut Table) -> i32 {
    let mut min = -((Board::WIDTH * Board::HEIGHT - position.nb_moves()) as i32) / 2;
    let mut max = ((Board::WIDTH * Board::HEIGHT + 1 - position.nb_moves()) as i32) / 2;

    while min < max {
        let mut med = min + (max - min) / 2;
        if med <= 0 && min / 2 < med {
            med = min / 2;
        } else if med >= 0 && max / 2 > med {
            med = max / 2;
        }
        let result = negamax(position, table, med, med + 1);
        if result <= med {
            max = result;
        } else {
            min = result;
        }
    }
    return min;
}

// at least alpha, at most beta
pub fn negamax(position: Board, table: &mut Table, mut alpha: i32, mut beta: i32) -> i32 {
    let possible = position.nonlosing_moves();
    //if possible == 0 {
    //    return -(((Board::WIDTH * Board::HEIGHT - position.nb_moves())/2) as i32)
    //}
    if position.nb_moves() == Board::HEIGHT * Board::WIDTH {
        return 0;
    }

    if position.can_win_next() {
        let mut score = Board::HEIGHT * Board::WIDTH + 1 - position.nb_moves();
        score /= 2; // allows encoding for different players is symmetric
        return score as i32;
    }
    

    //let max = ((Board::WIDTH * Board::HEIGHT - 1 - position.nb_moves()) / 2) as i32;
    let max;
    match table.get(&position) {
        Some(score) => max = score + Board::MIN_SCORE - 1,
        None => max = ((Board::WIDTH * Board::HEIGHT - 1 - position.nb_moves()) / 2) as i32,
    }

    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    /*
    let mut move_sort = MoveSort::new();
    for i in 0..Board::WIDTH {
        let action = possible & Board::col_mask(i);
        if action > 0 {
            let value = position.action_score(action);
            move_sort.insert(action, value);
        }
    }


    while move_sort.size > 0 {
        let action = move_sort.getNext().action;
        let mut next_position = position.copy();
        next_position.play(action);
        let score = -negamax(next_position, table, -beta, -alpha);
        if score >= beta {
            return score;
        }
        if score > alpha {
            alpha = score;
        }
    }
    */
    for i in 0..Board::WIDTH {
        if position.can_play(Board::exploration_order[i as usize]) {
            let action = Board::exploration_order[i as usize];
            let mut next_position = position.copy();
            next_position.play_col(action);
            let score = -negamax(next_position, table, -beta, -alpha);
            if score >= beta {
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }
    }

    table.add(&position, alpha - Board::MIN_SCORE + 1);

    return alpha;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MoveNode {
    action: u64,
    value: i32,
}

// sort from minimum to maximum
struct MoveSort {
    pub moves: Vec<MoveNode>,
    size: usize,
}

impl MoveSort {
    fn new() -> MoveSort {
        let moves = vec![
            MoveNode {
                action: 0,
                value: 0
            };
            7
        ];
        MoveSort {
            moves: moves,
            size: 0,
        }
    }

    fn clear(&mut self) {
        self.size = 0;
    }

    fn insert(&mut self, action: u64, value: i32) {
        self.size += 1;
        // increase the size
        // shift things over by 1 until you find a move with a bigger value or you have shifted everything over
        // then insert at that location
        let mut current = self.size - 1;
        while current > 0 && self.moves[current - 1].value > value {
            self.moves[current] = self.moves[current - 1];
            current -= 1;
        }
        self.moves[current].action = action;
        self.moves[current].value = value;
    }

    fn getNext(&mut self) -> MoveNode {
        self.size -= 1;
        self.moves[self.size]
    }
}

#[derive(Clone, Copy)]
struct TableNode {
    node: u64,
}

impl TableNode {
    const hash_mask: u64 = (1_u64 << 49) - 1;

    fn get_value(&self) -> i8 {
        (self.node & ((1_u64 << 8) - 1)) as i8
    }

    fn get_hash(&self) -> u64 {
        self.node >> 8
    }

    fn new(key: u64, value: i8) -> TableNode {
        TableNode {
            node: (key << 8) | (value as u64),
        }
    }
}

pub struct Table {
    contents: Vec<TableNode>,
    add_count: i32,
    get_count: i32,
}

impl Table {
    const CAPACITY: u64 = 8000000 * 2;

    pub fn new() -> Table {
        let empty = TableNode { node: 0 };
        Table {
            contents: vec![empty; Table::CAPACITY as usize],
            add_count: 0,
            get_count: 0,
        }
    }

    fn add(&mut self, position: &Board, score: i32) {
        self.add_count += 1;
        let index = (position.hash() % Table::CAPACITY) as usize;
        self.contents[index] = TableNode::new(position.hash(), score as i8);
    }

    fn get(&mut self, position: &Board) -> Option<i32> {
        self.get_count += 1;
        let index = (position.hash() % Table::CAPACITY) as usize;
        let node = self.contents[index];
        if node.get_hash() == position.hash() {
            return Some(node.get_value() as i32);
        } else {
            return None;
        }
    }

    pub fn clear(&mut self) {
        let empty = TableNode { node: 0 };
        for i in 0..Table::CAPACITY {
            self.contents[i as usize] = empty;
        }
    }

    pub fn results(&self) {
        println!(
            "There have been {} adds and {} gets",
            self.add_count, self.get_count
        );
    }
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
    fn table() {
        let board = Board::construct("162636");
        let mut table = Table::new();
        table.add(&board, 20);
        assert_eq!(table.get(&board), Some(20));
        table.add(&board, 10);
        assert_eq!(table.get(&board), Some(10));

        table.results();
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

    #[test]
    fn move_sorter() {
        let mut sorter = MoveSort::new();
        sorter.insert(0, 0);
        sorter.insert(5, 5);
        sorter.insert(1, 1);
        sorter.insert(4, 4);
        sorter.insert(2, 2);
        sorter.insert(3, 3);
        //let mut action = sorter.getNext();
        for i in (0..6).rev() {
            assert_eq!(
                sorter.getNext(),
                MoveNode {
                    action: i,
                    value: i as i32
                }
            );
        }
    }
}