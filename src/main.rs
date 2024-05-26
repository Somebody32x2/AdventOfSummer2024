use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Failure to Read File");
    let mut sum: i32 = 0;
    for line in input.split("\n") {
        let mut mostRecentNum: Option<u8> = Option::None;
        for char in line.chars() {
            if char.is_numeric() {
                if mostRecentNum.is_none() {
                    sum += (char as u8 - '0' as u8) as i32 * 10;
                }
                mostRecentNum = Option::Some(char as u8 - '0' as u8);
            }
        }
        sum += (mostRecentNum.unwrap_or(0) as i32);
    }
    println!("{}", sum);
}
