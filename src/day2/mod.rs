use std::collections::HashMap;
use std::fs;

pub(crate) fn solve() {
    let input: String = fs::read_to_string("src\\day2\\input.txt").expect("Failure to Read File");

    let max_cubes: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)].into_iter().collect();

    let mut game_id_sum: u32 = 0;
    for mut line in input.split("\n") {
        let game_strs: Vec<&str> = line.split(":").collect();
        let game_id: u32 = game_strs[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let mut valid_game = true;
        for pull_str in game_strs[1].split(";").collect::<Vec<&str>>() {
            for color_num_str in pull_str.split(",").collect::<Vec<&str>>() {
                let (num_str, color) = color_num_str.trim().split_once(" ").unwrap();
                println!("#{game_id}: num_str {}, color {}", num_str, color);
                let num: u32 = num_str.parse().unwrap();
                if num > *max_cubes.get(color).unwrap() {
                    valid_game = false;
                }
            }
        }
        if valid_game {
            game_id_sum += game_id;
        }
    }
    println!("{}", game_id_sum);
}
