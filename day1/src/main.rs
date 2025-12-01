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
    let mut count = 0;
    let mut dial = START;

    let content = fs::read_to_string(FILE_PATH).unwrap();
    let lines_content: Vec<&str> = content.lines().collect();

    lines_content.into_iter().for_each(|l| {
        let (direction, rotation) = l.split_at(1);
        println!("dial : {dial:?} / direction : {direction:?} / rotation: {rotation:?}");

        dial = rotate(dial, direction.into(), rotation.parse().unwrap()).unwrap();

        if dial == 0 {
            count += 1;
        }
        
    });

    println!("answer: {count}");
}

/// Rotate the dial and return the result
/// 
/// # Arguments
/// - dial: actual state of the dial before rotation
/// - direction: which direction to turn the dial
/// - rotation: number of time to rotate the dial
fn rotate(mut dial: i16, direction: Direction, rotation: i16) -> Result<i16, i16> {
    match direction {
        Direction::LEFT => {
            dial = dial - rotation;

            while dial < MIN {
                dial = dial + MAX;
            }
            dial
        }
        Direction::RIGHT => {
            dial = dial + rotation;

            while dial >= MAX {
                dial = dial - MAX;
            };
            dial
        }
    };

    if dial < MIN || dial > MAX {
        Err(dial)
    }else{
        Ok(dial)
    }
}

#[test]
fn rotation_tests(){
    // The dial starts by pointing at 50.
    // The dial is rotated L68 to point at 82.
    // The dial is rotated L30 to point at 52.
    // The dial is rotated R48 to point at 0.
    // The dial is rotated L5 to point at 95.
    // The dial is rotated R60 to point at 55.
    // The dial is rotated L55 to point at 0.
    // The dial is rotated L1 to point at 99.
    // The dial is rotated L99 to point at 0.
    // The dial is rotated R14 to point at 14.
    // The dial is rotated L82 to point at 32.

    let dial = 50;
    let dial = rotate(dial, Direction::LEFT, 68).unwrap();
    assert_eq!(dial, 82);
    
    let dial = rotate(dial, Direction::LEFT, 30).unwrap();
    assert_eq!(dial, 52);

    let dial = rotate(dial, Direction::RIGHT, 48).unwrap();
    assert_eq!(dial, 0);

    let dial = rotate(dial, Direction::LEFT, 5).unwrap();
    assert_eq!(dial, 95);

    let dial = rotate(dial, Direction::RIGHT, 60).unwrap();
    assert_eq!(dial, 55);

    let dial = rotate(dial, Direction::LEFT, 55).unwrap();
    assert_eq!(dial, 0);

    let dial = rotate(dial, Direction::LEFT, 1).unwrap();
    assert_eq!(dial, 99);

    let dial = rotate(dial, Direction::LEFT, 99).unwrap();
    assert_eq!(dial, 0);
    
    let dial = rotate(dial, Direction::RIGHT, 14).unwrap();
    assert_eq!(dial, 14);

    let dial = rotate(dial, Direction::LEFT, 82).unwrap();
    assert_eq!(dial, 32);

    //Stress tests: 
    let dial = rotate(0, Direction::LEFT, 550).unwrap();
    assert_eq!(dial, 50);

    let dial = rotate(0, Direction::RIGHT, 550).unwrap();
    assert_eq!(dial, 50);
}