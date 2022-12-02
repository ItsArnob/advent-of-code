// lots of lines for a simple puzzle
// decided to make it more readable and easy to understand
// rather than making it more compact.
// I'm not doing an in depth documentation, it should be self explanatory.

struct Opponent;
struct Me;
struct Points;

impl Opponent {
    pub const ROCK: &'static str = "A";
    pub const PAPER: &'static str = "B";
    pub const SCISSORS: &'static str = "C";
}
impl Me {
    pub const ROCK_OR_LOSE: &'static str = "X"; // ROCK in case of part one and LOSE in case of part two
    pub const PAPER_OR_DRAW: &'static str = "Y"; // same thing as above but for PAPER
    pub const SCISSORS_OR_WIN: &'static str = "Z"; // same thing as above but for SCISSORS
}
impl Points {
    pub const ROCK: i32 = 1;
    pub const PAPER: i32 = 2;
    pub const SCISSORS: i32 = 3;
    pub const WIN: i32 = 6;
    pub const DRAW: i32 = 3;
    pub const LOSE: i32 = 0;
}

pub fn run(data: &String) {

    let mut score_part1 = 0;
    let mut score_part2 = 0;
    let rounds = data.split("\n");

    for round in rounds {
        let round = round.trim().split(" ").collect::<Vec<&str>>();
        let opponent = round[0].trim();
        let me = round[1].trim();

        match opponent {
            Opponent::ROCK => {
                match me {
                    Me::ROCK_OR_LOSE => {
                        score_part1 += Points::DRAW + Points::ROCK;
                        score_part2 += Points::LOSE + Points::SCISSORS;
                    },
                    Me::PAPER_OR_DRAW => {
                        score_part1 += Points::WIN + Points::PAPER;
                        score_part2 += Points::DRAW + Points::ROCK;
                    },
                    Me::SCISSORS_OR_WIN => {
                        score_part1 += Points::LOSE + Points::SCISSORS;
                        score_part2 += Points::WIN + Points::PAPER;
                    },
                    _ => { invalid_data(); }
                }
            },
            Opponent::PAPER => {
                match me {
                    Me::ROCK_OR_LOSE => {
                        score_part1 += Points::LOSE + Points::ROCK;
                        score_part2 += Points::LOSE + Points::ROCK;
                    },
                    Me::PAPER_OR_DRAW => {
                        score_part1 += Points::DRAW + Points::PAPER;
                        score_part2 += Points::DRAW + Points::PAPER;
                    },
                    Me::SCISSORS_OR_WIN => {
                        score_part1 += Points::WIN + Points::SCISSORS;
                        score_part2 += Points::WIN + Points::SCISSORS;
                    },
                    _ => { invalid_data(); }
                }
            },
            Opponent::SCISSORS => {
                match me {
                    Me::ROCK_OR_LOSE => {
                        score_part1 += Points::WIN + Points::ROCK;
                        score_part2 += Points::LOSE + Points::PAPER;
                    },
                    Me::PAPER_OR_DRAW => {
                        score_part1 += Points::LOSE + Points::PAPER;
                        score_part2 += Points::DRAW + Points::SCISSORS;
                    },
                    Me::SCISSORS_OR_WIN => {
                        score_part1 += Points::DRAW + Points::SCISSORS;
                        score_part2 += Points::WIN + Points::ROCK;
                    },
                    _ => { invalid_data(); }
                }
            },
            _ => { invalid_data(); }
        }
    }

    println!("part 1: {}", score_part1);
    println!("part 2: {}", score_part2);
}

fn invalid_data() {
    println!("Exiting because: Invalid entry found in input data.");
    std::process::exit(0);
}