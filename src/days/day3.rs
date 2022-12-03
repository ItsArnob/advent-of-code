pub fn run(data: &String) {

    // create a vector (array) from data, split by new line. eg. ["abcdeaah", "ijklmi", "opqrstur", "qwertw", "loremipsuo", "asdfghjd"]
    let rucksacks = data.split("\n").collect::<Vec<&str>>();

    // made a copy so we can use it again, this is only because of rust and
    // has nothing to do with the logic of the puzzle.
    let rucksacks_copy = rucksacks.clone();

    // this is for part 2 of the puzzle.
    // create a vector of arrays, each array contains 3 elements(which is one group), each element is an elf.
    // we are using the `chunks` function to split the vector into groups of 3.
    // ie. [["abcdeaah", "ijklmi", "opqrstur"], ["qwertw", "loremipsuo", "asdfghjd"]]
    let groups = rucksacks_copy.chunks(3).collect::<Vec<&[&str]>>();

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    // part 1
    for rucksack in rucksacks {
        // according to the puzzle, first half of a rucksack represents items in the first compartment
        // and the second half represents items in the second compartment.
        // so we equally split the rucksack into two parts. eg. "abcdeaah" => ["abcd", "eaah"]
        let compartments = rucksack.trim().split_at(rucksack.len() / 2);

        // we find the common letter in the two compartments.
        // this filter function might return duplicate letters but they will be the same letter.
        // eg. ["abcd", "eaah"] => "aa"
        let common_letter_in_compartments = compartments.0.chars().filter(|c| compartments.1.contains(*c)).collect::<String>();

        // only get one char from the string. eg. f from "fff" or a from "aa".
        // with this we converted the string to a char type also.
        let  common_letter_in_char_type = common_letter_in_compartments.chars().next().unwrap();

        // self explanatory.
        // function defined below.
        let priority = get_priority_of_char(common_letter_in_char_type);
        sum_part1 += priority;
    }

    // part 2
    // this is more or less the same as part 1
    // but we are using the groups we created earlier.
    // each group is an array of 3 elements, each element is an elf.
    // eg. ["abdefg", "abjklj", "aljljl"]

    // we are going to compare first two elements of each group
    // and compare the result with the third.
    // when comparing the first two, we might get multiple common letters. eg. ["abdefg", "abjklj"] => "ab"
    // compare that to the third element. eg. ["ab", "aljljl"] => "a"
    for group in groups {
        let common_letter_in_first_two_groups = group[0].chars().filter(|c| group[1].contains(*c)).collect::<String>();
        let common_letter_between_first_two_and_the_last_group = group[2].chars().filter(|c| common_letter_in_first_two_groups.contains(*c)).collect::<String>();
        let common_letter_in_char_type = common_letter_between_first_two_and_the_last_group.chars().next().unwrap();
        let priority = get_priority_of_char(common_letter_in_char_type);
        sum_part2 += priority;
    }

    println!("part 1: {}", sum_part1);
    println!("part 2: {}", sum_part2);

}

// according to the puzzle, the priority of lower case letters are higher than the priority of upper case letters.
// a to z => 1 to 26 [this corresponds to the letter position in the alphabet]
// A to Z => 27 to 52 [this corresponds to the letter position in the alphabet + 26, since we have 26 lower case letters]
// so we add 26 to the letter position of upper case letters to get the priority.
fn get_priority_of_char(c: char) -> usize {
    return if c.is_uppercase() {
        get_char_pos(c) + 26
    } else {
        get_char_pos(c)
    }
}

// returns the letter position of the char in the alphabet.
// eg. a => 1, b => 2, c => 3, z => 26 or A => 1, B => 2, C => 3, Z => 26
// converting chars to usize returns their decimal place in the ascii table.
// they are organized in the a-z and A-Z order.
// so if we subtract the starting position of the first letter in the alphabet
// in the ascii table from the one of another letter + 1, we get the position of the letter in the alphabet.
// the starting position of lower case letters and upper case letters are different. so we use an if statement
// to check if the char is upper case or lower case.
fn get_char_pos(c: char) -> usize {
    let starting_position: usize;
    if c.is_uppercase() {
        starting_position = 'A' as usize;
    } else {
        starting_position = 'a' as usize;
    }

    let c_pos = c as usize;
    c_pos - starting_position + 1
}