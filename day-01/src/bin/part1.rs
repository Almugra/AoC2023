use std::time::Instant;

pub const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

fn main() {
    let input = include_str!("/home/mc/Code/advent-of-code-2023/day-01/inputs/day1-1.txt");
    let mut total = 0;

    let now = Instant::now();
    let mut str = String::default();
    for line in input.lines() {
        for char in line.chars() {
            match (char.is_numeric(), str.len()) {
                (true, 0) => {
                    str.push(char);
                    str.push(char);
                }
                (true, 2) => {
                    str.pop();
                    str.push(char);
                }
                _ => {}
            }
        }
        total += str.parse::<i32>().unwrap();
        str.clear();
    }

    println!("{:?}", now.elapsed());
    println!("{}", total);
}
