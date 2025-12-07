use core::panic;
use std::fs;


const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content = content.trim();
    let content: Vec<&str> = content.lines().collect();
    let content: Vec<Vec<&str>> = content.into_iter().map(|s| s.split("").filter(|s| !s.is_empty()).collect()).collect();
    
    // println!("content: {content:?}");

    let count = process(content);

    println!("result: {count}");
}

fn process(lines: Vec<Vec<&str>>) -> u32 {
    let mut count = 0;
    let mut beam = vec![];

    lines.into_iter().for_each(|line| {
        let (new_beam, add_count) = find_on_line(line, beam.clone());

        count += add_count;

        println!("{beam:?}");
        beam = new_beam;
    });

    count
}

fn find_on_line(line: Vec<&str>, mut beam: Vec<usize>) -> (Vec<usize>, u32) {
    let mut count = 0;

    line.into_iter().enumerate().for_each(|(k, c)| {
        match c {
            "." => {}, //Skip
            "S" => {
                if beam.len() != 0 {
                    panic!("beam should be zero when starting");
                }
        
                beam.push(k);
            },
            "^" => {
                let new_beam = split(beam.clone(), k);
                if new_beam != beam {
                    count += 1;
                }

                beam = new_beam;
            },
            e => panic!("beam has a not handled char: {e}")
        }
    });

    (beam, count)
}

fn split(mut beam: Vec<usize>, doko: usize) -> Vec<usize> {
    if beam.iter().any(|&b| b == doko) {
        println!("before:{beam:?}");
        
        beam = beam.into_iter().filter(|&s| s != doko).collect();
        beam.push(doko-1);
        beam.push(doko+1);

        println!("after:{beam:?}");
    }

    beam.dedup();
    beam
}



#[test]
fn process_test() {
    let lines: Vec<Vec<&str>> = vec![
        vec![".", ".", ".", ".", ".", ".", ".", "S", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", "^", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", "^", ".", "^", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", "^", ".", "^", ".", "^", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", ".", "^", ".", "^", ".", ".", ".", "^", ".", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", ".", "^", ".", "^", ".", ".", ".", "^", ".", "^", ".", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", ".", "^", ".", ".", ".", "^", ".", ".", ".", ".", ".", "^", ".", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
        vec![".", "^", ".", "^", ".", "^", ".", "^", ".", "^", ".", ".", ".", "^", "."],
        vec![".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", "."],
    ];

    assert_eq!(process(lines), 21);
}