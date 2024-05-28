use std::fs;
use colored::Colorize;

pub(crate) fn solve() {
    let input: String = fs::read_to_string("src\\day3\\input.txt").expect("Failure to Read File");
     let symbols: Vec<char> = "+*=-&#/%$@".chars().collect();

    let mut part_sum: u32 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let char_grid: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();
    let max_x = lines[0].len() - 1;
    let max_y = lines.len() - 1;
    for (mut y, mut line) in lines.iter().enumerate() {
        let mut is_current_part_number = false;
        let mut current_num_str: String = "".to_string();

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                current_num_str.push(char);
                // println!("x:{x} / {max_x}, y: {y} / {max_y}");
                // Check for surrounding symbols
                if x > 0 && y > 0          && symbols.contains(&char_grid[y - 1][x - 1]) { is_current_part_number = true } // tl
                if y > 0                   && symbols.contains(&char_grid[y - 1][x])     { is_current_part_number = true } // tm
                if x < max_x && y > 0      && symbols.contains(&char_grid[y - 1][x + 1]) { is_current_part_number = true } // tr
                if x > 0                   && symbols.contains(&char_grid[y][x - 1])     { is_current_part_number = true } // l
                if x <max_x                && symbols.contains(&char_grid[y][x + 1])     { is_current_part_number = true } // r
                if x > 0 && y < max_y      && symbols.contains(&char_grid[y + 1][x - 1]) { is_current_part_number = true } // bl
                if y < max_y               && symbols.contains(&char_grid[y + 1][x])     { is_current_part_number = true } // bm
                if x < max_x && y < max_y  && symbols.contains(&char_grid[y + 1][x + 1]) { is_current_part_number = true } // br


            }
            if !char.is_numeric() || x == max_x  {
                if is_current_part_number {
                    part_sum += current_num_str.parse::<u32>().unwrap();

                    print!("{}", current_num_str.green());
                } else {
                    print!("{}", current_num_str.red())
                }
                is_current_part_number = false;
                current_num_str = "".to_string();

                if symbols.contains(&char) {
                    print!("{}", char.to_string().cyan());
                } else {
                    print!("{}", char);
                }

            }
        }
        println!()


    }
    println!("{}", part_sum);
}
// 538293