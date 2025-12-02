// https://adventofcode.com/2025/day/2

use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let mut all_invalids: Vec<u64> = vec![];
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content = content.trim_end();

    let split_content: Vec<&str> = content.split(',').collect();

    let content_range: Vec<(&str, &str)> = split_content.into_iter().map(|c| {
        let split: Vec<&str> = c.split('-').collect();

        let ([v1], [v2]) = split.split_at(1) else {panic!("Value is not a range : {split:?}")};

        (*v1, *v2)
    }).collect();

    for (from, to) in content_range {
        // println!("from: {from:?} / to: {to:?}");
        let mut invalids = find_invalid(from.parse().unwrap(), to.parse().unwrap());
        
        all_invalids.append(&mut invalids);
    }
    
    let all_invalids_sum: u64 = all_invalids.into_iter().sum();
    
    println!("\n all invalids: {all_invalids_sum}");
}

/// Returns the invalid values from a range
/// 
/// # Arguments
/// - from: range start value
/// - to: range stop value (included)
/// 
/// # Returns
/// All invalid values
fn find_invalid(from: u64, to: u64) -> Vec<u64> {
    let mut invalids = vec![];

    //Loop into all numbers in range
    for current in from..to+1 {
        //Convert current to a vector of each chars
        let current_string = current.to_string();
        let chars: Vec<char> = current_string.chars().collect();
        let chars: Vec<String> = chars.into_iter().map(|c| c.to_string()).collect();

        let mut is_ok = false;
        let mut divide = 1;

        //Loop until you can't split in more than one chunk
        while divide < chars.len() {
            //create chunks with chars
            let chunks:Vec<&[String]> = chars.chunks(divide).collect();
            // println!("chunks: {chunks:?}");
            
            is_ok = true;
            let mut old_chunk: Option<String> = None;

            //Loop through all chunks
            for c in chunks {
                let chunk = c.join("");

                // println!("merged_chunk: {chunk:?}");
                // println!("old_chunk: {old_chunk:?}\n");

                match old_chunk {
                    None => old_chunk = Some(chunk),
                    Some(ref oc) => {
                        if chunk == *oc {
                            old_chunk = Some(chunk);
                        }else{
                            is_ok = false;
                        }
                    }
                }
            }

            // println!("is_ok? {is_ok:?}");

            if is_ok { break };

            divide += 1;
        }
        
        if is_ok {
            // println!("invalid: {current:?}.");
            invalids.push(current);
        }
    };

    invalids
}

#[test]
fn find_invalid_test() {
    let invalids = find_invalid(11, 22);
    assert_eq!(invalids.iter().count(), 2);
    assert_eq!(invalids.iter().find(|&&i| i == 11), Some(&11));
    assert_eq!(invalids.iter().find(|&&i| i == 22), Some(&22));

    let invalids = find_invalid(95, 115);
    assert_eq!(invalids.iter().count(), 2);

    let invalids = find_invalid(998, 1012);
    assert_eq!(invalids.iter().count(), 2);

    let invalids = find_invalid(1188511880, 1188511890);
    assert_eq!(invalids.iter().count(), 1);

    let invalids = find_invalid(222220, 222224);
    assert_eq!(invalids.iter().count(), 1);

    let invalids = find_invalid(1698522, 1698528);
    assert_eq!(invalids.iter().count(), 0);

    let invalids = find_invalid(446443, 446449);
    assert_eq!(invalids.iter().count(), 1);

    let invalids = find_invalid(38593856, 38593862);
    assert_eq!(invalids.iter().count(), 1);
    
    let invalids = find_invalid(565653, 565659);
    assert_eq!(invalids.iter().count(), 1);

    let invalids = find_invalid(824824821, 824824827);
    assert_eq!(invalids.iter().count(), 1);

    let invalids = find_invalid(2121212118, 2121212124);
    assert_eq!(invalids.iter().count(), 1);
}