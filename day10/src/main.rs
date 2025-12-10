use std::fs;

const FILE_PATH: &str = "./input";

fn main() {
    let content = fs::read_to_string(FILE_PATH).unwrap();
    let content = content.trim();
    let content: Vec<&str> = content.lines().collect();
    let content:Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<u32>)> = content.iter().map(|&c| {
        let c: Vec<&str> = c.split("]").collect();
        assert_eq!(c.len(), 2);

        let lights: Vec<&str> = c[0].split("").collect();
        let mut lights: Vec<&str> = lights.into_iter().filter(|&s| !s.is_empty()).collect();
        
        lights.remove(0);
        let lights: Vec<bool> = lights.into_iter().map(|s| s == "#").collect();


        let c: Vec<&str> = c[1].split("{").collect();
        assert_eq!(c.len(), 2);

        // let joltages = c[1]
        let joltages: Vec<&str> = c[1].split(",").collect();
        let mut joltages: Vec<&str> = joltages.into_iter().filter(|&s| !s.is_empty()).collect();
        joltages.remove(joltages.len()-1);
        let joltages = joltages.into_iter().map(|s| s.parse().unwrap()).collect();
        
        let buttons: Vec<&str> = c[0].trim().split(" ").collect();
        let buttons: Vec<&str> = buttons.iter().map(|&s| {
            let (_, s) = s.split_at(1);
            let (s, _) = s.split_at(s.len()-1);
            s
        }).collect();
        
        let buttons: Vec<Vec<usize>> = buttons.iter().map(|&s| {
            let s: Vec<&str> = s.split(",").collect();
            s.iter().map(|&u| u.parse().unwrap()).collect()
        }).collect();

        (lights, buttons, joltages)
    }).collect();

    let mut total = 0;

    // let t = content.len();
    content.iter().for_each(|(lights, buttons, _joltages)| {

        println!("lights: {lights:?} - buttons: {buttons:?}");
        let c = find_fewest_press(lights.to_owned(), buttons.to_owned());
        total += c.len();
        println!("total: {total:?}");
    });
    
    println!("result: {total:?}");
}

/// Returns the buttons for the fewest press
fn find_fewest_press(lights: Vec<bool>, buttons: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let all_buttons_possibilities: Vec<Vec<Vec<usize>>> = cext::iter::all_order_permutations(buttons); 
    println!("all_buttons_possibilities: {all_buttons_possibilities:?}");
    let mut best_press_possibility: Vec<Vec<usize>> = vec![];

    println!("found all possibilities for this one");

    all_buttons_possibilities.iter().for_each(|possibility| {
        let mut current_press: Vec<Vec<usize>> = vec![];
        let mut current_lights = lights.clone();
        possibility.iter().for_each(|buttons| {
            buttons.iter().for_each(|&button| {
                press_button(&mut current_lights, button);
            });

            current_press.push(buttons.to_vec());

            if current_lights.iter().all(|&l| l == false) {
                if current_press.len() < best_press_possibility.len() || best_press_possibility.len() == 0 {
                    best_press_possibility = current_press.clone();
                }
            }
        });
    });

    best_press_possibility
}

fn press_button(lights: &mut Vec<bool>, button: usize) {
    lights[button] = !lights[button]
}

#[test]
fn find_fewest_press_test() {
    let lights = vec![false, true, true, false];
    let buttons = vec![
        vec![3],
        vec![1,3],
        vec![2],
        vec![2,3],
        vec![0,2],
        vec![0,1]
    ];

    assert_eq!(find_fewest_press(lights, buttons).len(), 2);

    let lights = vec![false, false, false, true, false];
    let buttons = vec![
        vec![0, 2, 3, 4],
        vec![2, 3],
        vec![0, 4],
        vec![0, 1, 2],
        vec![1, 2, 3, 4]
    ];

    assert_eq!(find_fewest_press(lights, buttons).len(), 3);

    let lights = vec![false, true, true, true, false, true];
    let buttons = vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 3, 4],
        vec![0, 1, 2, 4, 5],
        vec![1, 2]
    ];

    assert_eq!(find_fewest_press(lights, buttons).len(), 2);
}