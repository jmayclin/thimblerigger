// I wonder whether 8 capacity would make things
// faster?

mod board;
mod solver;
mod sort;
mod table;

use board::Board;
use solver::solve;
use std::fs;
use std::time::Instant;
use table::Table;


fn generate_cache(board: String, depth: u8, table: &mut Table) {
    if depth == 0 {
        return;
    }
    for i in 0..Board::WIDTH {
        let mut board2 = Board::construct(&board);
        board2.play_col(i);
        let now = Instant::now();
        let (result, mut action) = solve(board2, table);
        action += 1;
        if now.elapsed().as_secs_f32() > 0.25 {
            println!("\"{}{}\":[{},{}]", board, i + 1, action, result,);
        }
        if !board2.can_win_next() {
            let mut board3 = board2;
            board3.play_col((action - 1) as u8);
            generate_cache(format!("{}{}{}", board, i + 1, action), depth - 1, table);
        }
    }
}

pub fn do_the_magic(state: &str) -> (i32, i32) {
    let board = Board::construct(state);
    let mut table = Table::new();
    let (result, mut action) = solve(board, &mut table);
    action += 1;
    (result, action)
}

fn play_game(init: &str) {
    let mut table = Table::new();
    let mut board = Board::construct(init);
    board.display();
    let mut stop: bool = board.can_win_next();
    while !stop {
        let now = Instant::now();
        let (result, mut action) = solve(board, &mut table);
        action += 1;
        board.play_col(action as u8);
        board.display();
        println!(
            "{},{},{},{}",
            action,
            result,
            now.elapsed().as_millis(),
            board.nb_moves() / 2
        );
        stop = board.can_win_next();
    }
}

fn evaluate_test_sets() {
    let files = vec![
        "test_cases/Test_L3_R1",
        "test_cases/Test_L2_R1",
        "test_cases/Test_L2_R2",
        "test_cases/Test_L1_R1",
        "test_cases/Test_L1_R2",
        "test_cases/Test_L1_R3",
    ];
    let _count = 0;
    for file in files {
        println!("calculating for {}", file);
        let mut count = 0;
        let mut table = Table::new();
        let now = Instant::now();
        for line in fs::read_to_string(file).unwrap().lines() {
            count += 1;
            let mut itr = line.split(' ');
            let state = itr.next().unwrap();
            let board = Board::construct(state);
            let expect = itr.next().unwrap().parse::<i32>().unwrap();
            //let min = -100;
            //let max = 100;
            //let result = thimblerigger::negamax(board, &mut table, min, max);
            let (result, action) = solve(board, &mut table);
            //table.clear();
            if expect != result {
                println!(
                    "Your incompetence is astounding. State: {} was {} but should be {}",
                    state, result, expect
                );
                panic!();
            }
            println!("result of {} achieved by playing {}", result, action);

            if !(0..=6).contains(&action) {
                panic!();
            }
            if file == "test_cases/Test_L1_R3" {
                println!(
                    "\taverage time of {} microseconds from {} trials",
                    now.elapsed().as_micros() / count,
                    count
                );
            }
        }
        println!(
            "{} | average time of {} microseconds from {} trials",
            file,
            now.elapsed().as_micros() / count,
            count
        );
    }
}

fn main() {
    //evaluate_test_sets();
    println!("Hello, world!");
    //let mut table = Table::new();
    //let result = do_the_magic("444447");
    //println!("{:?}", result)
    play_game("444345443332565356")
}
