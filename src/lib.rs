mod board;
mod solver;
mod sort;
mod table;

use board::Board;
use solver::solve;
use std::fs;
use std::time::Instant;
use table::Table;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn do_the_magic(state: String) -> String {
    let board = Board::construct(&state);
    let mut table = Table::new();
    let (result, action) = solve(board, &mut table);
    let (result, mut action) = solve(board, &mut table);
    action += 1;
    format!("{{\"utility\":{},\"action\":{}}}", result, action)
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
