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

    let accessible_rolls = find_rolls(rolls);

    let sum: usize = accessible_rolls
        .into_iter()
        .map(|lines| lines.into_iter().filter(|&c| c == true).count())
        .sum();

    println!("result = {sum:?}");
}

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
    let rolls = vec![
        vec![ false, false, true, true, false, true, true, false, true, false, ],
        vec![ true, false, false, false, false, false, false, false, false, false, ],
        vec![ false, false, false, false, false, false, true, false, false, false, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ true, false, false, false, false, false, false, false, false, true, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ true, false, false, false, false, false, false, false, false, false, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ true, false, true, false, false, false, false, false, true, false, ],
    ];

    let accessible_rolls = find_rolls(rolls);

    let sum: usize = accessible_rolls
        .into_iter()
        .map(|lines| lines.into_iter().filter(|&c| c == true).count())
        .sum();

    assert_eq!(sum, 13);



    let rolls = vec![
        vec![ true, false, true, true, false, true, true, false, true, false, ],
        vec![ true, false, false, false, false, false, false, false, false, false, ],
        vec![ false, false, false, false, false, false, true, false, true, true, ],
        vec![ false, false, false, false, false, false, false, false, false, true, ],
        vec![ true, false, false, true, false, false, false, false, false, true, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ false, false, false, false, false, false, false, false, false, false, ],
        vec![ true, false, false, false, false, false, false, false, false, true, ],
        vec![ false, false, false, false, false, false, false, false, true, true, ],
        vec![ true, false, true, false, false, false, false, false, true, true, ],
    ];

    let accessible_rolls = find_rolls(rolls);
    
    // println!("accessible_rolls: {accessible_rolls:?}");

    let sum: usize = accessible_rolls
        .into_iter()
        .map(|lines| lines.into_iter().filter(|&c| c == true).count())
        .sum();


    assert_eq!(sum, 20);
}
