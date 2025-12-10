// https://adventofcode.com/2025/day/5

use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();

    //Sanitize and split in two vars
    let mut ranges: Vec<&str> =  content.trim().split("\n\n").collect();
    let ids = ranges.split_off(1);
    
    let ranges: String = ranges.into_iter().flat_map(|s| s.chars()).collect();
    let ids: String = ids.into_iter().flat_map(|s| s.chars()).collect();
    
    let ranges: Vec<&str> = ranges.split("\n").collect();
    let ranges: Vec<Vec<&str>> = ranges.into_iter().map(|s| s.split("-").collect()).collect();
    let ranges: Vec<Vec<u64>> = ranges.into_iter().map(|range| range.into_iter().map(|s| s.parse().unwrap()).collect()).collect();
    
    let ids: Vec<&str> = ids.split("\n").collect();
    let ids: Vec<u64> = ids.into_iter().map(|s| s.parse().unwrap()).collect();

    //todo: fix ranges, issue: range is 

    
    let result= find_fresh(ids, ranges).iter().count();
    
    println!("{result}");

}

fn merge_range(r1: Vec<u64>, r2: Vec<u64>) -> Vec<u64> {
    //Need to check if it overlaps before and after
    
    let new_range;

    if r1[0] > r2[0] {
        //e.g. 100, 50
    }else if r2[1] > r2[1] {
        //e.g. 100, 50
        
    }else if r1[0] < r2[0] {
        //e.g. 50, 100
        
    }else if r1[1] < r2[1] {
        //e.g. 50, 100
        
    }else{
        
    }

    new_range
}

fn find_fresh(ids: Vec<u64>, ranges: Vec<Vec<u64>>) -> Vec<u64>{
    let mut freshs: Vec<u64> = vec![];

    ids.into_iter().for_each(|id| { ranges.iter().for_each(|range| {
        if id > range[0] && id <= range[1] {
            freshs.push(id);
        }
    }); });

    // ranges.iter().for_each(|range| {
    //     println!("range: {range:?}");
    //     let start: u64 = range[0];
    //     let end: u64 = range[1];
    //     for fresh_id in start..=end {
    //         if let Some(found_id) = ids.iter().find(|&&id| id == fresh_id) {
    //             freshs.push(found_id.to_owned());
    //         }
    //         // ids.iter().for_each(|&find_id| {
    //         //     println!("here: {fresh_id}");
    //         //     if find_id == fresh_id {
    //         //         println!("pushing: {fresh_id}");
    //         //         freshs.push(&fresh_id);
    //         //     }
    //         // });
    //     }
    // });

    //Remove dup
    freshs.dedup();
    
    freshs
}

#[test]
fn find_fresh_test() {
    let ranges: Vec<Vec<u64>> = vec![
        vec![3, 5],
        vec![10, 14],
        vec![16, 20],
        vec![12, 18],
    ];

    let ids: Vec<u64> = vec![
        1,
        5,
        8,
        11,
        17,
        32,
    ];

    let freshs = find_fresh(ids, ranges);

    assert_eq!(freshs, vec![5, 11, 17]);
}

fn merge_range_test() {
    let r1 = vec![100, 200];
    let r2 = vec![150, 250];

    assert_eq!(merge_range(r1, r2), vec![100, 250]);
    
    let r1 = vec![100, 200];
    let r2 = vec![50, 150];

    assert_eq!(merge_range(r1, r2), vec![50, 200]);
}