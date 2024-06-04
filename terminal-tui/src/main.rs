mod utils;

use std::env::args;
use std::fs::read_to_string;
use serde::Deserialize;
use terminal::{solver, Answer, Guess, Result, Solver};
use utils::{get_inter, get_word, read_index};


#[derive(Deserialize)]
struct JsonResponse{
    words: Vec<String>,
}

fn run() -> Result<()> {
    let path = args().nth(1).ok_or("Usage terminal <PATH>")?;
    let content = read_to_string(path)?;
    let response:JsonResponse = serde_json::from_str(&content)?;
    let words = response.words;
    let mut solver = Solver::new(words, Vec::new())?;
    let mut answer = solver.answer();
    loop{
        let word = get_word(&answer);
        let inter = get_inter();
        answer = solver.next_answer(&(word, inter));
    }
}



fn main() {
    if let Err(e) = run() {
        println!("{e}");
    }
}
