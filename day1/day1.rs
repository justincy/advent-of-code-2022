use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let mut calories: Vec<i32> = Vec::new();
    let mut current = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(text) = line {
                if text.is_empty() {
                    calories.push(current);
                    current = 0;
                } else {
                    current += text.parse::<i32>().unwrap()
                }
            }
        }
    }

    calories.sort();
    calories.reverse();
    calories.truncate(3);

    let sum: i32 = calories.iter().sum();

    println!("Calories: {sum}");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
