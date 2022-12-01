pub fn run(data: &String) {

    let mut calories: Vec<i32> = Vec::new(); // this one stores the sum of all elves.
    let mut last_number = 0; // only needed for the loop.
    let mut last_highest_calorie= 0;
    let mut second_last_highest_calorie = 0;
    let mut third_last_highest_calorie = 0;

    let mut numbers_iterable = data.split("\n").peekable();

    // this loops over the number in the data and pushes the sum of calories of each elf to the calories vec.
    // explanation: Iterate over the numbers and add it to last_number (summation).
    // while iterating over the numbers there will be a "number" that will be zero in length because its the empty
    // line that separates the elves. We check for that zero length "number" and if true, what remains of last_number
    // is the total calorie count of that elf's food. we then push that to the calories vector and reset last_number to zero for the next elf.

    while let Some(number) = numbers_iterable.next() {

        let number_trimmed = number.trim(); // to trim line returns (as in CRLF/LF)

        if number_trimmed.len() == 0 {
            calories.push(last_number);
            last_number = 0;
        }
        last_number += number_trimmed.parse().unwrap_or(0); // set to 0 so nothing gets added if number_trimmed is an empty new line.

        // this checks for the last iteration and if it's not a new line.
        // in the last iteration we push the final elf's calories since this wont get covered by the
        // previous if statement.
        if numbers_iterable.peek().is_none() && number_trimmed.len() != 0 {
            calories.push(last_number);
        }
    }

    // this loop is to find the 3 highest calorie count of the elves.
    // last_highest_calorie is the highest calorie food one of the elves have.
    // second_last_highest_calorie is the second highest calorie food another one of the elves have.
    // third_last_highest_calorie is the third highest calorie food another one of the elves have.
    // explanation: self-explanatory 🦀🦀

    for  elf_calories in calories {
        if elf_calories > last_highest_calorie {
            third_last_highest_calorie = second_last_highest_calorie;
            second_last_highest_calorie = last_highest_calorie;
            last_highest_calorie = elf_calories;

        } else if elf_calories > second_last_highest_calorie {
            third_last_highest_calorie = second_last_highest_calorie;
            second_last_highest_calorie = elf_calories;
        } else if elf_calories > third_last_highest_calorie {
            third_last_highest_calorie = elf_calories;
        }
    }

    println!("part 1: {}",last_highest_calorie);
    println!("part 2: {}", last_highest_calorie + second_last_highest_calorie + third_last_highest_calorie);
}