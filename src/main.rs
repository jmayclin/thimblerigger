// I wonder whether 8 capacity would make things
// faster?
use std::error::Error;
use std::fs;
use std::time::Instant;
use thimblerigger::Board;

fn main() {
    println!("Hello, world!");
    let mut board = Board::construct("2252576253462244111563365343671351441");

    let now = Instant::now();
    let mut count = 0;
    for line in fs::read_to_string("Test_L3_R1").unwrap().lines() {
        count += 1;
        let mut itr = line.split(" ");
        let state = itr.next().unwrap();
        board = Board::construct(&state);
        let expect = itr.next().unwrap().parse::<i32>().unwrap();
        let result = thimblerigger::negamax(&board);
        if expect != result {
            println!("an check {} {} {}", state, result, expect);
        }
        println!("an check {} {} {}", state, result, expect);
    }
    println!("evaluated on {} trials", count);
    println!("time per trials is {}", now.elapsed().as_micros() / count);
}
