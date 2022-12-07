// I ABSOLUTELY DO NOT WANT TO DOCUMENT THIS. THIS HURTS MY BRAIN

pub fn run(data: &String) {


    let lines = data.split("\n").collect::<Vec<&str>>();
    let mut stacks: Vec<Vec<String>> = Vec::new();


    let mut skip_value = 1;
    let mut iteration_count = 0;
    let mut stack_count = 0;

    while {
        let mut stack: Vec<String> = Vec::new();

        if iteration_count != 0 {
            skip_value += 4;

        }

        for line in lines.clone() {
        

            let crate_ = line.chars().skip(skip_value).take(1);
            let string = crate_.clone().collect::<String>();
            if line.trim() == "" {
                break;
            }
            let _stack_count = String::from(line.trim().chars().last().unwrap()).parse::<i32>();
            if _stack_count.is_ok() {
                stack_count = _stack_count.unwrap();
                break;
            }
            if string.trim() == "" {
                continue
            }
            stack.push(string);
        }
        stacks.push(stack);
        iteration_count += 1;

        iteration_count != stack_count
    } {};

    let lines_a = data.split("\n").collect::<Vec<&str>>();
    let instructions = lines_a.into_iter().skip(stack_count as usize + 1).clone().map(|x| x.trim()).collect::<Vec<&str>>();

    let mut stacks_part1 = stacks.clone();
    let mut stacks_part2 = stacks.clone();
    for instruction in instructions {
        if instruction == "" {
            continue;
        }

        let move_count1 = instruction.split("move ").skip(1).collect::<String>();
        let move_count  : usize = move_count1.split(" ").take(1).collect::<String>().parse().unwrap();
        let from_stack1 = move_count1.split("from ").skip(1).collect::<String>();
        let from_stack: usize = from_stack1.split(" ").take(1).collect::<String>().parse().unwrap();
        let to_stack1 = move_count1.split("to ").skip(1).collect::<String>();
        let to_stack: usize = to_stack1.split(" ").take(1).collect::<String>().parse().unwrap();

        let moved_crate = stacks_part1[from_stack - 1].clone().into_iter().take(move_count as usize).collect::<Vec<String>>();
        let mut moved_crate2 = stacks_part2[from_stack - 1].clone().into_iter().take(move_count as usize).collect::<Vec<String>>();
        stacks_part1[from_stack - 1].drain(0..move_count as usize);
        stacks_part2[from_stack - 1].drain(0..move_count as usize);
        for crate_ in moved_crate {
            stacks_part1[to_stack -1].insert(0, crate_);

        }
        moved_crate2.reverse();
        for crate_ in moved_crate2 {
            stacks_part2[to_stack -1].insert(0, crate_);

        }

    }
    let mut result_part1 = String::new();
    let mut result_part2 = String::new();
    for stack in stacks_part1 {
        if stack.len() != 0 {
            result_part1.push_str(stack[0].as_str());
        }
    }
    for stack in stacks_part2 {
        if stack.len() != 0 {
            result_part2.push_str(stack[0].as_str());
        }
    }
    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);

}