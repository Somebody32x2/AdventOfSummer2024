use std::fs;
const NUM_STRS: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];
const NUM_REP_STRS: [&str; 10] = [ // This is a bad solution but works without significant compute required for substring matching
    "z0ero",
    "o1ne",
    "t2wo",
    "t3hree",
    "fo4ur",
    "fi5ve",
    "s6ix",
    "se7ven",
    "eig8ht",
    "ni9ne"
];
pub(crate) fn solve() {
    let mut input: String = fs::read_to_string("src\\day1\\input.txt").expect("Failure to Read File");
    // Part 1.5
    for (i, num) in NUM_STRS.iter().enumerate() {
        input = input.replace(*num, NUM_REP_STRS[i]);
    }
    
    let mut sum: i32 = 0;
    for line in input.split("\n") {
        let mut most_recent_num: Option<u8> = None;
        for char in line.chars() {
            if char.is_numeric() {
                if most_recent_num.is_none() {
                    sum += (char as u8 - '0' as u8) as i32 * 10;
                }
                most_recent_num = Some(char as u8 - '0' as u8);
            }
        }
        sum += (most_recent_num.unwrap_or(0) as i32);
    }
    println!("Day 1 Solution: {}", sum);
}
