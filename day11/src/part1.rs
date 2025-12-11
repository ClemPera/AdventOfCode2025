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

            println!("paths: {paths:?}");

            (device, paths)
        })
        .collect();

    let result = find_all_paths(&devices, &"you".to_string());

    println!("{result:?}");
}

/// Find all paths
///
/// # Arguments:
/// Device and output linked : Vec<(Device, Vec<Output>)>
///
/// # Returns:
/// Number of paths
fn find_all_paths(devices: &Vec<(String, Vec<String>)>, current: &String) -> u32 {
    let mut number_of_paths = 0;

    devices.iter().for_each(|(device, paths)| {
        if device == current {
            paths.iter().for_each(|path| {
                if path == "out" {
                    number_of_paths += 1
                } else if path == "you" && current != "you" {
                    //Ignore to not make an infinite loop
                } else {
                    number_of_paths += find_all_paths(devices, path)
                }
            });
        }
    });

    number_of_paths
}

#[test]
fn find_all_paths_test() {
    let devices: Vec<(String, Vec<String>)> = vec![
        (
            "aaa".to_string(),
            vec!["you".to_string(), "hhh".to_string()],
        ),
        (
            "you".to_string(),
            vec!["bbb".to_string(), "ccc".to_string()],
        ),
        (
            "bbb".to_string(),
            vec!["ddd".to_string(), "eee".to_string()],
        ),
        (
            "ccc".to_string(),
            vec!["ddd".to_string(), "eee".to_string(), "fff".to_string()],
        ),
        ("ddd".to_string(), vec!["ggg".to_string()]),
        ("eee".to_string(), vec!["out".to_string()]),
        ("fff".to_string(), vec!["out".to_string()]),
        ("ggg".to_string(), vec!["out".to_string()]),
        (
            "hhh".to_string(),
            vec!["ccc".to_string(), "fff".to_string(), "iii".to_string()],
        ),
        ("iii".to_string(), vec!["out".to_string()]),
    ];

    let paths = find_all_paths(&devices, &"you".to_string());

    assert_eq!(paths, 5)
}
