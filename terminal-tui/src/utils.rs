use std::io::{stdin, Stdin};

use terminal::Answer;

/// Get a word from terminal
pub fn get_word(answer: &Answer) -> String{
    println!("Choose a word from list");
    let max_index = answer.len() - 1;
    let answer = answer.iter().collect::<Vec<_>>();
    for (i,(word, cost)) in answer.iter().enumerate(){
            println!("{i:2}. {word:15}, cost {cost}");
    }
    let i = read_index(max_index);
    answer[i].0.clone()
}

pub fn get_inter() -> usize{
    println!("Enter number of intersections:");
    read_index(100)
}

/// read usize from stdin and check it in the range
pub fn read_index_range(stdin: &Stdin, range: &[usize]) -> usize {
    loop {
        let mut buf = String::new();
        if let Ok(_s) = stdin.read_line(&mut buf) {
            if let Ok(i) = buf.trim().parse::<usize>() {
                if range.iter().any(|&el| el == i) {
                    return i;
                } else {
                    println!("Index out of range, try again")
                }
            } else {
                println!("Error parsing index, try again")
            }
        } else {
            println!("Someghing wrong, try again")
        }
    }
}

pub fn read_index(max_index: usize) -> usize {
    let range: Vec<_> = (0..max_index).collect();
    read_index_range(&stdin(), &range)
}