use std::{fs};

fn main() {
    
    let data = fs::read_to_string("/Users/gdboling/Projects/3gcodes/AdventOfCode2022/aoc-d1/data.txt").expect("should have been able to read file");

    let mut current_count:i32 = 0;
    let mut elfs = Vec::new();

    for line in data.lines() {
        if line.len() == 0 {
            elfs.push(current_count);
            current_count = 0;
        } else {
            let cnt:i32 = line.parse().unwrap();
            current_count = current_count + cnt;
        }
    }


    elfs.sort_by(|a, b| b.cmp(a));

    println!("{}", elfs[0] + elfs[1] + elfs[2]);

}
