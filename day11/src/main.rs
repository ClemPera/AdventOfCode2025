use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content: Vec<&str> = content.lines().collect();

    let devices: Vec<(String, Vec<String>)> = content
        .iter()
        .map(|&s| {
            let vec_s: Vec<&str> = s.split(":").collect();
            assert_eq!(vec_s.len(), 2);

            let device = vec_s[0].to_string();
            let paths: Vec<String> = vec_s[1]
                .split(" ")
                .filter(|&s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            // println!("paths: {paths:?}");

            (device, paths)
        })
        .collect();

    let mut visited = vec![];
    let result = find_paths(&devices, &"svr".to_string(), 0, false, false, &mut visited);

    println!("{result:?}");
}

/// Find all paths that visit fft and dac and go out
///
/// # Arguments:
/// - devices: Device and output linked : Vec<(Device, Vec<Output>)>
/// - current: name of the device where to search path in
/// - nested: number of time this function has been called recurcively
///
/// # Returns:
/// - u32: Number of paths that visit fft and go out
/// - bool: is fft been visited
/// - bool: is dac been visited
fn find_paths(devices: &Vec<(String, Vec<String>)>, current: &String, mut nested: u32, mut has_visited_fft: bool, mut has_visited_dac: bool, too_much_nested:  &mut Vec<String>) -> u32 {
    let mut number_of_paths = 0;

    if nested > 610 {
        println!("i'm too much nested: {current}");
        too_much_nested.push(current.clone());
    }else if nested > 500 {
        println!("500");
    }else if nested > 400 {
        println!("400");
    }else if nested > 300 {
        println!("300");
    }else if nested > 200 {
        println!("200");
    }else if nested > 100 {
        println!("100");
    }
    
    devices.iter().for_each(|(device, paths)| {
        if device == current {
            paths.iter().for_each(|path| {
                nested+=1;

                // println!("i'm {path} and i'm nested like: {nested}");
                if path == "dac" {
                    has_visited_dac = true;
                }else if path == "fft"{
                    has_visited_fft = true;
                }

                if path == "out" && has_visited_fft && has_visited_dac {
                    number_of_paths += 1
                } else if path == "svr" && current != "svr" {
                    //Ignore to avoid infinite loop
                } else if !too_much_nested.contains(path) {
                    number_of_paths += find_paths(devices, path, nested, has_visited_fft, has_visited_dac, too_much_nested);
                }
            });
        }
    });
    
    // if current == "svr" {
    //     println!("end of run: {number_of_paths}, fft: {has_visited_fft}, dac: {has_visited_dac}")
    // }

    number_of_paths
}

#[test]
fn find_paths_test() {
    let devices: Vec<(String, Vec<String>)> = vec![
        ("svr".to_string(), vec!["zzz".to_string(), "bbb".to_string(), "aaa".to_string()]),
        ("aaa".to_string(), vec!["fft".to_string()]),
        ("fft".to_string(), vec!["ccc".to_string()]),
        ("bbb".to_string(), vec!["tty".to_string()]),
        ("tty".to_string(), vec!["ccc".to_string()]),
        ("ccc".to_string(), vec!["ddd".to_string(), "eee".to_string()]),
        ("ddd".to_string(), vec!["hub".to_string()]),
        ("hub".to_string(), vec!["fff".to_string()]),
        ("eee".to_string(), vec!["dac".to_string()]),
        ("dac".to_string(), vec!["fff".to_string()]),
        ("fff".to_string(), vec!["ggg".to_string(), "hhh".to_string()]),
        ("ggg".to_string(), vec!["out".to_string()]),
        ("hhh".to_string(), vec!["out".to_string()]),
        ("xxx".to_string(), vec!["yyy".to_string()]),
        ("yyy".to_string(), vec!["zzz".to_string(), "xxx".to_string()]),
        ("zzz".to_string(), vec!["xxx".to_string(), "yyy".to_string()])
    ];

    let mut visited = vec![];
    let paths = find_paths(&devices, &"svr".to_string(), 0, false, false, &mut visited);

    assert_eq!(paths, 2);
}