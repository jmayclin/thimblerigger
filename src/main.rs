// I wonder whether 8 capacity would make things
// faster?
use std::error::Error;
use std::fs;
use std::time::Instant;
use thimblerigger::{Board, Table};

fn main() {
    println!("Hello, world!");

    let mut board;
    board = Board::construct("5255312676617552463");
    let mut table = Table::new();
    board.display();
    let result = thimblerigger::solve(board, &mut table);
    println!("{}", result);
    let mut count = 0;

    let files = vec![
        "test_cases/Test_L3_R1",
        "test_cases/Test_L2_R1",
        "test_cases/Test_L2_R2",
        "test_cases/Test_L1_R1",
        "test_cases/Test_L1_R2",
        "test_cases/Test_L1_R3",
    ];

    for file in files {
        println!("calculating for {}", file);
        let mut count = 0;
        let mut table = Table::new();
        let now = Instant::now();
        for line in fs::read_to_string(file).unwrap().lines() {
            count += 1;
            let mut itr = line.split(" ");
            let state = itr.next().unwrap();
            board = Board::construct(&state);
            let expect = itr.next().unwrap().parse::<i32>().unwrap();
            //let min = -100;
            //let max = 100;
            //let result = thimblerigger::negamax(board, &mut table, min, max);
            let result = thimblerigger::solve(board, & mut table);
            if expect != result {
                println!(
                    "Your incompetence is astounding. State: {} was {} but should be {}",
                    state, result, expect
                );
                panic!();
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
