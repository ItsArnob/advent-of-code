use std::{env,fs, time};
pub mod days;

// Note: this piece of code only reads the file and executes the function for the specific puzzle.
// Code for the puzzles are located in the src/days directory.
// All the puzzle codes have a `run` function that takes in the input data as a string and solves the puzzle.

fn main() {
    let now = time::Instant::now();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 { provide_valid_args() }

    let day = args[1].clone();
    let data_path = args[2].clone();

    if  day.trim().len() == 0 || !data_path.trim().len() ==0 { provide_valid_args() }

    match fs::read_to_string(data_path) {
        Ok(data) => {
            match &day[..] {
                "1" => {
                    days::day1::run(&data);
                }
                "2" => {
                    days::day2::run(&data);
                }
                "3" => {
                    days::day3::run(&data);
                }
                _ => println!("Day argument must be between 1-3")
            }
        }
        Err(e) => {
            println!("Exiting because: {}", e.to_string())
        }
    }
    let elapsed = now.elapsed();
    println!("Execution time: {}ms", elapsed.as_millis());
}

fn provide_valid_args() {
    println!("Please provide valid args.");
    println!("1st argument must be the day in integer and second argument must be the path to the input file.");
    std::process::exit(0);
}