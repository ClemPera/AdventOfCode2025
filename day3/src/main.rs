// https://adventofcode.com/2025/day/3

use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content: Vec<&str> = content.lines().collect();
    println!("content: {content:?}");

    let content: Vec<Vec<u8>> = content
        .iter()
        .map(|&line| {
            let sl: Vec<&str> = line.split("").filter(|v| !v.is_empty()).collect();
            sl.iter().map(|p| p.parse().unwrap()).collect()
        })
        .collect();

    let result: Vec<u64> = content
        .into_iter()
        .map(|c| {
            println!("{c:?}");
            largest_joltage(c)
        })
        .map(|v| v.into())
        .collect();
    let result: u64 = result.into_iter().sum();

    println!("result: {result:?}");
}

fn largest_joltage(mut batteries: Vec<u8>) -> u64 {
    // let mut all_joltage: Vec<u64> = batteries.iter().map(|&n| n.into()).collect();

    // for (k,v) in batteries.iter().enumerate() {
    //     let mut t: Vec<usize> = vec![];

    //     for _ in 0..12 {
    //         batteries.iter().enumerate().for_each(|(k2,v2)| {
    //             if !t.iter().any(|&v3| v3 == k2) {
    //                 all_joltage.push(format!("{v}{v2}").parse().unwrap());

    //                 t.push(k);
    //             }
    //         });
    //     }
    // }

    // all_joltage.iter().max().unwrap().to_owned()

    // while batteries.len() > 12 {
    //     let k = batteries.iter().enumerate().min_by(|(k,v), (k2, v2)| v.cmp(v2)).map(|(k, v)| k).unwrap();
    //     batteries.remove(k);
    // }

    

    // batteries.into_iter().flatten();
    // let val:Vec<char> = batteries.into_iter().flat_map(|n| n.to_string().chars()).collect();
    // let t = val.into_iter().map(|s| s.to_string().parse().unwrap()).collect();

    // t

    //Loop from left to right -> another loop: remove min starting from right
    // while batteries.len() > 12 {
    //     let mut previousk: usize = 0;
    //     let mut previousv: u8 = 255;
    //     for (lnk, lnv) in batteries.clone().into_iter().enumerate() {
    //         println!("full: {batteries:?} | lnk: {lnk} | lnv: {lnv}");
            
    //         if previousv != 255 {
    //             if previousv <= lnv {
    //                 let r = batteries[lnk];
    //                 println!("pv:{previousv} | lnv:{lnv} | removing: {r}");
    //                 batteries.remove(lnk);
    //                 break;
    //             }
    //         }


    //         // for i in 1..10 {
                
    //         // batteries.remove(bk);
    //             // let bk = batteries.iter().rev().enumerate().map(|(k,v)| k).unwrap();
    //         // }
    //         previousk = lnk;
    //         previousv = lnv;
    //     }
    // }

    // let mut max: u64 = 0;

    // for (bk,bv) in batteries.iter().enumerate() {
    //     for (ik, iv) in batteries.iter().enumerate(){
            
    //         // while index <
    //         // let b2 = batteries.clone();
    //         // for (jk, jv) in b2.iter().enumerate() {

    //         // }
    //     }234234234234278

    // }

    // let mut filtered_batteries = vec![];
    
    // let mut lp = false;

    // while filtered_batteries.len() < 12 {
    //     let mut previousk: usize = 0;
    //     let mut previousv: u8 = 255;

    //     for (lnk, lnv) in batteries.clone().into_iter().enumerate() {
    //         println!("full: {batteries:?} | lnk: {lnk} | lnv: {lnv}");
            
    //         if previousv != 255 {
    //             if lnv < previousv || (lp && lnv <= previousv) {
    //                 let r = batteries[previousk];
    //                 println!("pv:{previousv} | lnv:{lnv} | adding: {r} | lp: {lp}");

    //                 filtered_batteries.push(previousv);
    //                 batteries.remove(previousk);
    //                 break;
    //             }
    //         }

    //         previousk = lnk;
    //         previousv = lnv;

    //         if lnk >= batteries.len()-1 {
    //             lp = true;
    //         }
    //     }
    // }


    //Loop de gauche à droite et prendre le plus gros chiffre until 12
    //Définition de "plus gros chiffre": c+1 < c

    
    println!("\n");

    // let bstr: Vec<String> = filtered_batteries.into_iter().map(|n| n.to_string()).collect();
    // let map: String = bstr.iter().map(|s| s.chars()).flatten().collect();

    // map.parse().unwrap()
}

#[test]
fn largest_joltage_test() {
    assert_eq!(
        largest_joltage(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
        987654321111
    );
    assert_eq!(
        largest_joltage(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
        811111111119
    );
    assert_eq!(
        largest_joltage(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
        434234234278
    );
    assert_eq!(
        largest_joltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
        888911112111
    );
}
