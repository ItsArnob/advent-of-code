use std::collections::HashSet;

pub fn run(data: &String) {
    
    let mut first_4_from_index: Vec<char> = Vec::new();
    let mut last_14_from_index: Vec<char> = Vec::new();

    let mut start_of_packet = 0;
    let mut start_of_message = 0;

    for (index, char) in data.chars().enumerate() {

        if start_of_packet == 0 {
        
            if first_4_from_index.len() != 4 {
                first_4_from_index.push(char);
            } else {
                let deduped = first_4_from_index.clone().into_iter().collect::<HashSet<char>>();
                
                if deduped.len() == 4 {
                    start_of_packet = index;
                } else {
                    first_4_from_index.remove(0);
                    first_4_from_index.push(char);
                }
            }
        }

        if start_of_message == 0 {
            if last_14_from_index.len() != 14 {
                last_14_from_index.push(char);
            } else {
                let deduped = last_14_from_index.clone().into_iter().collect::<HashSet<char>>();
                
                if deduped.len() == 14 {
                    start_of_message = index;
                } else {
                    last_14_from_index.remove(0);
                    last_14_from_index.push(char);
                }
            }
        }

        if start_of_message != 0 && start_of_packet != 0 { break }

    }

    println!("Part 1: {}", start_of_packet);
    println!("Part 2: {}", start_of_message)

}

// first working solution
#[allow(dead_code)]
fn more_elegant_but_somehow_slower_version(data: &String) {
    
    let mut start_of_packet = 0;
    let mut start_of_message = 0;
    
    for i in 0..data.len() {
        let first_4_char_from_index = data.chars().skip(i).take(4).collect::<HashSet<char>>();
        let first_14_char_from_index = data.chars().skip(i).take(14).collect::<HashSet<char>>();
        
        if first_4_char_from_index.len() == 4 && start_of_packet == 0  {
            start_of_packet = i + 4;
        }
        
        if first_14_char_from_index.len() == 14 && start_of_message == 0 {
            start_of_message = i + 14;
            break;
        }

    } 

    println!("Part 1: {}", start_of_packet);
    println!("Part 2: {}", start_of_message);

}