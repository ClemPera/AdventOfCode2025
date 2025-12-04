// https://adventofcode.com/2025/day/3

use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content: Vec<&str> = content.lines().collect();
    // println!("content: {content:?}");

    let content: Vec<Vec<u8>> = content
        .iter()
        .map(|&line| {
            let sl: Vec<&str> = line.split("").filter(|v| !v.is_empty()).collect();
            sl.iter().map(|p| p.parse().unwrap()).collect()
        })
        .collect();

    // let result: Vec<u64> = content
    //     .into_iter()
    //     .map(|c| {
    //         println!("{c:?}");
    //         largest_joltage(c)
    //     })
    //     .map(|v| v.into())
    //     .collect();
    // let result: u64 = result.into_iter().sum();

    // println!("result: {result:?}");
}

fn find_largest_in_12_bank(mut batteries: Vec<u8>) -> u64 {
    let mut r: Vec<u8> = vec![0u8; 256];
    
    for _ in batteries.clone() {
        let ((lj1, lj2),(k1,k2)) = largest_joltage(batteries.clone());

        r[k1] = lj1;
        r[k2] = lj2;

        let fr: Vec<&u8> = r.iter().filter(|&&v| v > 0).collect();
        if fr.len() >= 12 {
            break;
        }

        batteries[k1] = 0;
        batteries[k2] = 0;
    }

    let fr: Vec<&u8> = r.iter().filter(|&&v| v > 0).collect();
    if fr.len() < 12 {
        panic!("couldn't find largest joltages in this elements: {fr:?}");
    }

    let c: Vec<String> = fr.into_iter().map(|v| v.to_string()).collect();

    c.join("").parse().unwrap()
}


///Find the two largest joltages in a Vector
/// 
/// # Arguments
/// - batteries: vector of all batteries
/// 
/// # Returns
/// (a,b): - a: the two largest joltage
///        - b: the adress of each
fn largest_joltage(batteries: Vec<u8>) -> ((u8, u8),(usize,usize)) {
    let mut all_joltage: Vec<((u8, u8), (usize, usize))> = vec![];

    for (k,&v) in batteries.iter().enumerate() {
        for (k2, &v2) in batteries.iter().enumerate() {
            if k2 > k {
                all_joltage.push(((v,v2), (k,k2)));
            }
        }
    }

    all_joltage.into_iter().max_by(|((a1, a2),_), ((b1,b2),_)| {
        let v1: u8 = format!("{a1}{a2}").parse().unwrap();
        let v2: u8 = format!("{b1}{b2}").parse().unwrap();

        v1.cmp(&v2)
    }).unwrap().to_owned()
}

#[test]
fn largest_joltage_test() {
    assert_eq!( find_largest_in_12_bank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]), 987654321111 );
    assert_eq!( find_largest_in_12_bank(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]), 811111111119 );
    assert_eq!( find_largest_in_12_bank(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]), 434234234278 );
    assert_eq!( find_largest_in_12_bank(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]), 888911112111 );
}
