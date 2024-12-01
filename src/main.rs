mod day01;

use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let puzzle = args.next().unwrap().parse().unwrap();
    let day = args.next().unwrap().parse().unwrap();
    let input = fs::read_to_string(args.next().unwrap()).unwrap();
    let answer = match (puzzle, day) {
        (1, 1) => day01::puzzle1(&input).to_string(),
        (1, 2) => day01::puzzle2(&input).to_string(),

        _ => panic!("no puzzle {} for day {}", puzzle, day),
    };
    println!("{}", answer.trim_end());
}
