// https://adventofcode.com/2025/day/1

use core::panic;
use std::fs;

const FILE_PATH: &str = "./input";
const START: i16 = 50;
const MIN: i16 = 0;
const MAX: i16 = 100;

enum Direction {
    LEFT,
    RIGHT
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("Could not parse direction")
        }
    }
}

fn main() {
    let mut all_clicks = 0;
    let mut count = 0;
    let mut dial = START;

    let content = fs::read_to_string(FILE_PATH).unwrap();
    let lines_content: Vec<&str> = content.lines().collect();

    lines_content.into_iter().for_each(|l| {
        let (direction, rotation) = l.split_at(1);
        println!("dial : {dial:?} / direction : {direction:?} / rotation: {rotation:?}");

        let (new_dial, clicks) = rotate(dial, direction.into(), rotation.parse().unwrap()).unwrap();
        dial = new_dial;

        if dial == 0 {
            count += 1;
        }

        all_clicks += clicks;
    });

    println!("\n# Answers:\ncount:  {count}\nclicks: {all_clicks}");
}

/// Rotate the dial and return the result
/// 
/// # Arguments
/// - dial: actual state of the dial before rotation
/// - direction: which direction to turn the dial
/// - rotation: number of time to rotate the dial
/// 
/// # Returns
/// Ok(a, b) = a is the new dial, b is the number of clicks
/// Err(a) = a is the dial number
fn rotate(dial: i16, direction: Direction, rotation: i16) -> Result<(i16, u16), i16> {
    let mut new_dial;
    let mut clicks = 0;

    match direction {
        Direction::LEFT => {
            new_dial = dial - rotation;

            while new_dial < MIN {
                new_dial = new_dial + MAX;

                clicks += 1;
            }

            //Handle edge cases
            if new_dial == 0 {
                clicks += 1
            }

            if dial == 0 {
                clicks -= 1;
            }
        }
        Direction::RIGHT => {
            new_dial = dial + rotation;

            while new_dial >= MAX {
                new_dial = new_dial - MAX;
                clicks += 1
            }
        }
    };

    if new_dial < MIN || new_dial > MAX {
        Err(new_dial)
    }else{
        Ok((new_dial, clicks))
    }
}

#[test]
fn rotation_tests(){
    // The dial starts by pointing at 50.
    // The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
    // The dial is rotated L30 to point at 52.
    // The dial is rotated R48 to point at 0.
    // The dial is rotated L5 to point at 95.
    // The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
    // The dial is rotated L55 to point at 0.
    // The dial is rotated L1 to point at 99.
    // The dial is rotated L99 to point at 0.
    // The dial is rotated R14 to point at 14.
    // The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.


    let dial = 50;
    let (dial, clicks) = rotate(dial, Direction::LEFT, 68).unwrap();
    assert_eq!(dial, 82);
    assert_eq!(clicks, 1);
    
    let (dial, clicks) = rotate(dial, Direction::LEFT, 30).unwrap();
    assert_eq!(dial, 52);
    assert_eq!(clicks, 0);
    
    let (dial, clicks) = rotate(dial, Direction::RIGHT, 48).unwrap();
    assert_eq!(dial, 0);
    assert_eq!(clicks, 1);
    
    let (dial, clicks) = rotate(dial, Direction::LEFT, 5).unwrap();
    assert_eq!(dial, 95);
    assert_eq!(clicks, 0);
    
    let (dial, clicks) = rotate(dial, Direction::RIGHT, 60).unwrap();
    assert_eq!(dial, 55);
    assert_eq!(clicks, 1);
    
    let (dial, clicks) = rotate(dial, Direction::LEFT, 55).unwrap();
    assert_eq!(dial, 0);
    assert_eq!(clicks, 1);
    
    let (dial, clicks) = rotate(dial, Direction::LEFT, 1).unwrap();
    assert_eq!(dial, 99);
    assert_eq!(clicks, 0);
    
    let (dial, clicks) = rotate(dial, Direction::LEFT, 99).unwrap();
    assert_eq!(dial, 0);
    assert_eq!(clicks, 1);
    
    let (dial, clicks) = rotate(dial, Direction::RIGHT, 14).unwrap();
    assert_eq!(dial, 14);
    assert_eq!(clicks, 0);

    let (dial, clicks) = rotate(dial, Direction::LEFT, 82).unwrap();
    assert_eq!(dial, 32);
    assert_eq!(clicks, 1);

    //Stress tests: 
    let (dial, clicks) = rotate(0, Direction::LEFT, 550).unwrap();
    assert_eq!(dial, 50);
    assert_eq!(clicks, 5);

    let (dial, clicks) = rotate(0, Direction::RIGHT, 550).unwrap();
    assert_eq!(dial, 50);
    assert_eq!(clicks, 5);
}