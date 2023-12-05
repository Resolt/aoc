use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;

fn main() {

    // Open file
    let file: io::Result<File>;

    match env::args().last() {
        Some(file_name) => {
            file = File::open(file_name);
        }
        None => {
            println!("no file provided");
            exit(1)
        }
    }

    // Get content
    let mut content = String::new();
    match file {
        Ok(mut f) => {
            let _ = f.read_to_string(&mut content);
        }
        error => {
            println!("failed reading file: {error:?}")
        }
    }

    // part1
    let mut sum = 0;
    for line in content.lines() {
        sum += line_first_last_digit(line);
    }
    println!("Part1: {sum}");

    // part2
    println!("Part2: BLEGH");
}

fn line_first_last_digit(line: &str) -> u32 {
    let mut f = line.chars().filter(|c| c.is_digit(10));
    let first_digit = f.next().unwrap().to_digit(10).unwrap();
    match f.last() {
        Some(c) => {
            first_digit*10 + c.to_digit(10).unwrap()
        }
        None => {
            first_digit*10 + first_digit
        }
    }
}