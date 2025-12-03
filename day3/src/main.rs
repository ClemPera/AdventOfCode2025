// https://adventofcode.com/2025/day/3

use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content: Vec<&str> = content.lines().collect();
    println!("content: {content:?}");

    let content: Vec<Vec<u8>> = content.iter().map(|&line| {
        let sl: Vec<&str> = line.split("").filter(|v| !v.is_empty()).collect();
        sl.iter().map(|p| p.parse().unwrap()).collect()
    }).collect();

    let result: Vec<u32> = content.into_iter().map(|c| largest_joltage(c)).map(|v|v.into()).collect();
    let result: u32 = result.into_iter().sum();

    println!("result: {result:?}");
}

fn largest_joltage(batteries: Vec<u8>) -> u8 {
    let mut all_joltage: Vec<u8> = vec![];

    for (k,v) in batteries.iter().enumerate() {
        for (k2, v2) in batteries.iter().enumerate() {
            if k2 > k {
                all_joltage.push(format!("{v}{v2}").parse().unwrap());
            }
        }
    }

    all_joltage.iter().max().unwrap().to_owned()
}

#[test]
fn largest_joltage_test() {
    assert_eq!(largest_joltage(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]), 98);
    assert_eq!(largest_joltage(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]), 89);
    assert_eq!(largest_joltage(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]), 78);
    assert_eq!(largest_joltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]), 92);
}
