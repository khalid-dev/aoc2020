use std::path::Path;
use std::fs;

#[derive(Debug)]
struct MovementSlope {
    horizontal: usize,
    vertical: usize,
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn trees_by_slope_traversal(hill: &Vec<&str>, slope: &MovementSlope) -> usize {
    let line_length = hill[0].len();
    const TREE: char = '#';
    let mut position = Position {x: 0, y: 0 };
    let mut tree_count: usize = 0;

    while position.y + slope.vertical < hill.len() {
        position.x = (position.x + slope.horizontal) % line_length;
        position.y += slope.vertical;

        let char_at_position = hill[position.y].chars().nth(position.x).unwrap();
        if char_at_position == TREE {
            tree_count += 1;
        }
    }
    
    return tree_count;
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    println!("In file {}", INPUT_FILENAME);

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    let mut entries: Vec<&str> = contents.split('\n').collect();
    entries.pop();
    
    let slope = MovementSlope { horizontal: 3, vertical: 1 };

    println!("tree count: {}", trees_by_slope_traversal(&entries, &slope));

    //PART 2:

    let slope_1 = MovementSlope { horizontal: 1, vertical: 1 };
    let slope_2 = MovementSlope { horizontal: 3, vertical: 1 };
    let slope_3 = MovementSlope { horizontal: 5, vertical: 1 };
    let slope_4 = MovementSlope { horizontal: 7, vertical: 1 };
    let slope_5 = MovementSlope { horizontal: 1, vertical: 2 };
    let slopes = [slope_1, slope_2, slope_3, slope_4, slope_5];

    let tree_count_products = slopes.iter()
        .fold(1, |acc, x| acc * trees_by_slope_traversal(&entries, x));

    println!("product: {}", tree_count_products);
}
