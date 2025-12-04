//https://adventofcode.com/2025/day/4

use std::fs;

const FILE_PATH: &str = "./input";

//In the whole program, x and y might be consistantly reverted

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let lines:Vec<&str> = content.lines().collect();

    let rolls: Vec<Vec<bool>> = lines.into_iter().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        chars.into_iter().map(|c| {
            if c == '@' {
                true
            }else{
                false
            }
        }).collect()
    }).collect();

    let answer = process(rolls);
    
    println!("answer: {answer}");

    //Part 1:

    // let accessible_rolls = find_rolls(rolls);

    // let sum: usize = accessible_rolls
    //     .into_iter()
    //     .map(|lines| lines.into_iter().filter(|&c| c == true).count())
    //     .sum();

    // println!("result = {sum:?}");
}

/// Process the removal of rolls 
/// 
/// # Arguments
/// - rolls: matrix of all rolls
/// 
/// # Returns
/// The total of all rolls removed
fn process(mut rolls: Vec<Vec<bool>>) -> u32 {
    let mut all_removed: u32 = 0;

    loop {
        let accessible_rolls = find_rolls(rolls.clone());
        
        let (new_rolls, removed) = remove(rolls.clone(), accessible_rolls.clone());

        // println!("rolls: {rolls:?}");
        // println!("accessible_rolls: {accessible_rolls:?}");
        // println!("new_rolls: {new_rolls:?}");
        
        if removed == 0 {
            break;
        }

        rolls = new_rolls;
        all_removed += removed;
    } 

    all_removed
}


/// Remove accessible rolls
/// 
/// # Arguments
/// - rolls: matrix of all rolls
/// - accessible_rolls: matrix of all accessible rolls
/// 
/// # Returns
/// - rolls matrix with the accessible rolls removed
/// - How much rolls have been removed
fn remove(rolls: Vec<Vec<bool>>, accessible_rolls: Vec<Vec<bool>>) -> (Vec<Vec<bool>>, u32) {
    let mut removed: u32 = 0;
    let removed_rolls = rolls.into_iter().enumerate().map(|(x, line)| {
        line.into_iter().enumerate().map(|(y, roll)| {
            if roll {
                if accessible_rolls[x][y] {
                    removed += 1;
                    false
                }else{
                    true
                }
            }else{
                false
            }
        }).collect()
    }).collect();

    (removed_rolls, removed)
}

/// Find the rolls that are accessible
/// 
/// # Arguments
/// - rolls: matrix of all rolls
/// 
/// # Returns
/// Matrix of which rolls are accessible
fn find_rolls(rolls: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    rolls
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.into_iter()
                .enumerate()
                .map(|(y, &roll)| {
                    if roll {
                        is_roll_accessible(rolls.clone(), x, y)
                    }else{
                        false
                    }
                })
                .collect()
        })
        .collect()
}

/// Return if the roll is accessible or not
/// 
/// # Arguments
/// - rolls: matrix of all rolls
/// - roll_x: x of roll to check
/// - roll_y: y of roll to check
/// 
/// # Returns
/// Is roll accessible
fn is_roll_accessible(rolls: Vec<Vec<bool>>, roll_x: usize, roll_y: usize) -> bool {
    let mut tonari: u32 = 0;

    for x in -1..=1 {
        for y in -1..=1 {
                let cx: i32 = roll_x as i32 + x;
                let cy: i32 = roll_y as i32 + y;

                let rolls_x_len: u32 = rolls.len().try_into().unwrap();
                let rolls_y_len: u32 = rolls[0].len().try_into().unwrap();

                if roll_x == 8 && roll_y == 8 {
                    println!("- cx:{cx} - cy:{cy} (reversed) | rolls_x_len: {rolls_x_len:?} - rolls_y_len: {rolls_y_len}");
                }

                if (cx != roll_x as i32 || cy != roll_y as i32) &&
                    cx >= 0 && cy >= 0 && 
                    cx < rolls.len().try_into().unwrap() && cy < rolls[0].len().try_into().unwrap() 
                {
                    // if roll_x == 8 && roll_y == 8 {
                    //     println!("- cx:{cx} - cy:{cy} (reversed)");
                    // }
                    if rolls[cx as usize][cy as usize] {
                        if roll_x == 8 && roll_y == 8 {
                            println!("   -- checked");
                        }
                        tonari += 1;
                    }
                }
        }
    }

    if roll_x == 8 && roll_y == 8 {
        println!("tonari: **{tonari:?}**");
    }

    tonari < 4
}

#[test]
fn find_rolls_test() {
    let rolls: Vec<Vec<bool>> = vec![
        vec![false, false, true, true, false, true, true, true, true, false],
        vec![true, true, true, false, true, false, true, false, true, true],
        vec![true, true, true, true, true, false, true, false, true, true],
        vec![true, false, true, true, true, true, false, false, true, false],
        vec![true, true, false, true, true, true, true, false, true, true],
        vec![false, true, true, true, true, true, true, true, false, true],
        vec![false, true, false, true, false, true, false, true, true, true],
        vec![true, false, true, true, true, false, true, true, true, true],
        vec![false, true, true, true, true, true, true, true, true, false],
        vec![true, false, true, false, true, true, true, false, true, false],
    ];
    
    let all_removed = process(rolls);

    assert_eq!(all_removed, 43);


    //Part 1 tests: 

    // let sum: usize = accessible_rolls
    //     .into_iter()
    //     .map(|lines| lines.into_iter().filter(|&c| c == true).count())
    //     .sum();

    // assert_eq!(sum, 13);

    // let rolls = vec![
    //     vec![ true, false, true, true, false, true, true, false, true, false, ],
    //     vec![ true, false, false, false, false, false, false, false, false, false, ],
    //     vec![ false, false, false, false, false, false, true, false, true, true, ],
    //     vec![ false, false, false, false, false, false, false, false, false, true, ],
    //     vec![ true, false, false, true, false, false, false, false, false, true, ],
    //     vec![ false, false, false, false, false, false, false, false, false, false, ],
    //     vec![ false, false, false, false, false, false, false, false, false, false, ],
    //     vec![ true, false, false, false, false, false, false, false, false, true, ],
    //     vec![ false, false, false, false, false, false, false, false, true, true, ],
    //     vec![ true, false, true, false, false, false, false, false, true, true, ],
    // ];

    // let accessible_rolls = find_rolls(rolls);
    
    // // println!("accessible_rolls: {accessible_rolls:?}");

    // let sum: usize = accessible_rolls
    //     .into_iter()
    //     .map(|lines| lines.into_iter().filter(|&c| c == true).count())
    //     .sum();


    // assert_eq!(sum, 20);
}
