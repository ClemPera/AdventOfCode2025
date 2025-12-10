use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content = content.trim();
    let content: Vec<&str> = content.lines().collect();
    
    let calculs = content.last().unwrap();
    let calculs: Vec<&str> = calculs.split_whitespace().collect();
    
    let (numbers, _) = content.split_at(content.len()-1);
    let numbers: Vec<Vec<&str>> = numbers.into_iter().map(|&s| s.split_whitespace().collect()).collect();

    let result = resolve_problems(calculs, numbers);
    let result: u64 = result.into_iter().sum();

    println!("{result}");
}

fn resolve_problems(calculs: Vec<&str>, numbers: Vec<Vec<&str>>) -> Vec<u64> {
    let mut result:Vec<u64> = vec![0u64; calculs.len()];
    let mut resolved: Vec<Vec<u64>> = vec![vec![0u64; numbers.len()]; calculs.len()];

    calculs.iter().enumerate().for_each(|(k,_calcul)| {
        numbers.iter().for_each(|number| {
            resolved[k].push(number[k].parse().unwrap());
        });
    });

    
    println!("numbers: {numbers:?}");
    resolved.into_iter().enumerate().for_each(|(k,n)| {
        if calculs[k] == "+" {
            result[k] = n.iter().sum();
        }else {
            // let bla = result[k];
            // println!("{k}:{bla:?}");
            // println!("{n:?}");
            n.iter().for_each(|&rn|{
                if result[k] == 0 {
                    result[k] = rn;
                }else{
                    result[k] *= rn;
                }
            });
        }
    });

    result
}

#[test]
fn resolve_problems_test() {
    let calculs = vec!["*","+","*","+"];

    let numbers: Vec<Vec<&str>> = vec![
        vec!["123", "328", "51", "64"],
        vec!["45", "64", "387", "23"],
        vec!["6", "98", "215", "314"],
    ];

    let result = resolve_problems(calculs, numbers);

    assert_eq!(result, vec![33210, 490, 4243455, 401])
}