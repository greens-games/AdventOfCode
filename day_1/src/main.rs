use std::{fs::{File, self}, io::Lines};
use std::io::{ BufRead, BufReader };


fn main() {
    let f = File::open("res/input.txt").unwrap();

    // part_1(&f);
    part_2();
}

fn part_1(f:&File ) {
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

fn part_2() {
    let input = include_str!("../res/input.txt");

    let v:Vec<Vec<i32>> = input.split("\r\n\r")
    .map(|s| {
        s.split("\r\n").map(|s1| { 
            s1.trim().parse::<i32>().unwrap()
        }).collect()
    }).collect();
    let mut v2:Vec<i32> = v.into_iter().map(|nums| {
        nums.into_iter().sum()
        }).collect();

    v2.sort();
    println!("{:?}",v2);
    println!("{}",v2[v2.len()-1]+v2[v2.len()-2]+v2[v2.len()-3])
}