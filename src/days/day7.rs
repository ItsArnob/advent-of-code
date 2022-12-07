use std::collections::HashMap;

pub fn run(data: &String) {
    const SIZE_LIMIT: i32 = 100000;
    let lines = data.split("\n");
    let mut path_hierarchy: Vec<String> = Vec::new();
    let mut directories: HashMap<String, i32> = HashMap::new();

    for line in lines {
        let result = line.trim().split(" ").collect::<Vec<&str>>();

        // a command executed by the user
        if result[0] == "$" {
            if result[1] == "cd" && result[2] == ".." {
                path_hierarchy.pop();
            } else if result[1] == "cd" {
                path_hierarchy.push(result[2].to_string())
            } else { continue; }

        } else if result[0] == "dir" { continue; }
        else {
            let size: i32 = result[0].parse().unwrap();
            let mut i = 0;
            while i < path_hierarchy.len() {
                let open_dir_path = &path_hierarchy[0..i + 1].join("/");
                directories.entry(open_dir_path.clone())
                    .and_modify(|original_size| *original_size += size)
                    .or_insert(size);
                i += 1;
            }
        }
    }

    

    let mut result_part1 = 0;
    let space_taken = directories.get("/").unwrap();
    let free_space = 70000000 - space_taken;
    let need_to_free = 30000000 - free_space;
    

    // start off with a bigger number so we can compare it to the actual values and find the smaller ones.
    // using space_taken here but it could've been any large number but we know that the amount we can delete 
    // wont be larger than the amount of space taken.
    let mut result_part2 = space_taken; 
    for size in directories.values() {

        if size <= &SIZE_LIMIT {
            result_part1 += size;
        }

        if size > &need_to_free && size < result_part2 {
            result_part2 = size;
        }
    }

    println!("part 1: {}", result_part1);
    println!("part 2: {}", result_part2);

}
