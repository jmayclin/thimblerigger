use super::board::Board;
use super::table::Table;
use super::sort::MoveSort;

pub fn solve(position: Board, table: &mut Table) -> (i32, i32) {
    let mut min = -((Board::WIDTH * Board::HEIGHT - position.nb_moves()) as i32) / 2;
    let mut max = ((Board::WIDTH * Board::HEIGHT + 1 - position.nb_moves()) as i32) / 2;
    let mut action = -100;
    if position.can_win_next() {
        let mut score = Board::HEIGHT * Board::WIDTH + 1 - position.nb_moves();
        score /= 2; // allows encoding for different players is symmetric
        return (score as i32, position.winning_move() as i32);
    }
    while min < max {
        let mut med = min + (max - min) / 2;
        if med <= 0 && min / 2 < med {
            med = min / 2;
        } else if med >= 0 && max / 2 > med {
            med = max / 2;
        }
        let (result, action_c) = negamax(position, table, med, med + 1);
        //println!("\t{}", action_c);
        if action_c != -1 {
            action = action_c;
        }
        if result <= med {
            max = result;
        } else {
            min = result;
        }
    }
    return (min, action);
}

// at least alpha, at most beta
pub fn negamax(position: Board, table: &mut Table, mut alpha: i32, mut beta: i32) -> (i32, i32) {

    let possible = position.nonlosing_moves();
    //println!("{:b}", possible);
    if possible == 0 {
        let mut score = Board::HEIGHT * Board::WIDTH - position.nb_moves();
        score /= 2; // allows encoding for different players is symmetric
        return (-(score as i32), position.possible_move() as i32);
        // return forced move if available, or any move if not
    }

    if position.nb_moves() >= Board::HEIGHT * Board::WIDTH - 2 {
        return (0, position.possible_move() as i32);
    }



    let min = -(((Board::WIDTH * Board::HEIGHT - 2 - position.nb_moves()) / 2) as i32);
    if alpha < min {
        alpha = min;
        if alpha > beta {
            return (alpha, -1); //this should never be chosen so is shouldn't matter?
        }
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
            return (beta, -1);
        }
    }

    
    let mut move_sort = MoveSort::new();
    for i in (0..Board::WIDTH).rev() {
        let action = possible & Board::col_mask(Board::exploration_order[i as usize]);
        if action > 0 {
            let value = position.action_score(action);
            move_sort.insert(Board::exploration_order[i as usize], value);
        }
    }

    let mut best_action: i32 = -1;
    while move_sort.size > 0 {
        let action = move_sort.get_next();
        let mut next_position = position;
        next_position.play_col(action);
        let (mut score, step) = negamax(next_position, table, -beta, -alpha);
        score = -score;
        if score >= beta {
            return (score, action as i32);
        }
        if score > alpha {
            alpha = score;
            best_action = action as i32;
        }
    }
    /*
    for i in 0..Board::WIDTH {
        if possible & Board::col_mask(Board::exploration_order[i as usize]) != 0 {
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
    }*/

    table.add(&position, alpha - Board::MIN_SCORE + 1);

    return (alpha, best_action);
}


