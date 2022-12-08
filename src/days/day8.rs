pub fn run(data: &String) {

    let lines = data.split("\n").collect::<Vec<_>>();

    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut visible_trees_part1 = 0;
    let mut highest_scenic_score_part2 = 0;
    
    // create a grid representation of the input using vectors and also convert them to integers. 
    for line in lines {
        let row = line.trim().chars().map(|char| char.to_digit(10).unwrap()).collect::<Vec<u32>>();
        grid.push(row);
    }

    for column_index in 0..grid[0].len() {
        let column = grid.iter().map(|row| row[column_index]).collect::<Vec<u32>>();
        
        
        // first column is always visible from the left side
        // last column is always visible from the right side.
        if column_index == 0 || column_index + 1 == grid[0].len() {
            visible_trees_part1 += column.len();
            continue;
        }

        for (row_index, tree) in column.iter().enumerate() {
            let row = &grid[row_index];
            
            // top most tree is always visible from the top
            // bottom most tree is always visible from the bottom
            if row_index == 0 || row_index + 1 == column.len() {
                visible_trees_part1 += 1;
                continue;
            }

            // using + 1 to skip the current tree.
            let bottom_trees = &column[row_index + 1..column.len()];
            let top_trees = &column[0..row_index];
            let left_trees = &row[0..column_index];
            let right_trees = &row[column_index + 1..row.len()];

            // part 1
            // check if the tree is visible from ANY one of the sides.
            if is_side_visible(tree, bottom_trees) || is_side_visible(tree, top_trees) || is_side_visible(tree, left_trees) || 
            is_side_visible(tree, right_trees) {
                visible_trees_part1 += 1;
            }

            // part 2
            // reverse top and left so its ordered FROM the tree
            let top_viewing_distance = tree_viewing_distance(tree, top_trees, true);
            let bottom_viewing_distance = tree_viewing_distance(tree, bottom_trees, false);
            let left_viewing_distance = tree_viewing_distance(tree, left_trees, true);
            let right_viewing_distance = tree_viewing_distance(tree, right_trees, false);

            let score = top_viewing_distance * bottom_viewing_distance * left_viewing_distance * right_viewing_distance;
            if score > highest_scenic_score_part2 {
                highest_scenic_score_part2 = score;
            }
        }
    }
    
    println!("part 1: {}", visible_trees_part1);
    println!("part 2: {}", highest_scenic_score_part2);
   
}

fn is_side_visible(tree: &u32, side_trees: &[u32]) -> bool {
    !side_trees.iter().any(|side_tree| side_tree >= tree)
}
fn tree_viewing_distance(tree: &u32, side_trees: &[u32], reverse: bool) -> u32 {
    let mut last_tree = 0;
    let mut viewing_distance = 0;
    
    let trees = if reverse {
        let mut copy_trees = side_trees.to_vec();
        copy_trees.reverse();
        copy_trees
    } else { side_trees.to_vec() };
    
    for side_tree in trees {
        if side_tree >= last_tree || side_tree < *tree {
            viewing_distance += 1;
        }
        if side_tree >= *tree { break; }
        last_tree = side_tree;
    }
    viewing_distance
}