use std::{fs::{File}};
use std::io::{ BufRead, BufReader };


fn main() {
    let f = File::open("res/input.txt").unwrap();
    let lines = BufReader::new(f).lines();
    let mut sum:i32 = 0;
    let mut max = 0;
    for line in lines {
        let s = line.unwrap();
        
        if s.is_empty() {
            max = max.max(sum);
            sum = 0;
        }
        else {
            sum += s.parse::<i32>().unwrap();
        }
    }
    println!("{}",max);
}
