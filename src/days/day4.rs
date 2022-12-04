pub fn run(data: &String) {
    let section_assignment_pairs = data.split("\n").collect::<Vec<&str>>();
    let mut overlaps_part1 = 0;
    let mut overlaps_part2 = 0;

    for pairs in section_assignment_pairs {
        let pair = pairs.trim().split(",").collect::<Vec<&str>>();

        let pair_1 = pair[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let pair_2 = pair[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let pair_1_starting_position = pair_1[0];
        let pair_1_ending_position = pair_1[1];
        let pair_2_starting_position = pair_2[0];
        let pair_2_ending_position = pair_2[1];

        // part 1, checks if a pair range contains the other.
        // we check if the first pair is contained within the second pair.
        if range_contain((pair_1_starting_position, pair_1_ending_position), (pair_2_starting_position, pair_2_ending_position)) {
            overlaps_part1 += 1;
        }
        // if not we check if the second pair is contained within the first pair.
        else if range_contain((pair_2_starting_position, pair_2_ending_position), (pair_1_starting_position, pair_1_ending_position)) {
            overlaps_part1 += 1;
        }

        // part 2, checks overlap
        // for overlaps we only need to check if a pair's starting number is within the other pair's range.
        // check if the second pair's start is within the first pair.
        if range_overlap((pair_1_starting_position, pair_1_ending_position), pair_2_starting_position) {
            overlaps_part2 += 1;
        }
        // if not we check if the first pair's start is within the second pair.
        else if range_overlap((pair_2_starting_position, pair_2_ending_position), pair_1_starting_position) {
            overlaps_part2 += 1;
        }

    }
    println!("part 1: {}", overlaps_part1);
    println!("part 2: {}", overlaps_part2);

}

fn range_overlap(range: (i32, i32), value: i32) -> bool {
    value >= range.0 && value <= range.1
}

fn range_contain(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 >= range2.0 && range1.1 <= range2.1
}