use std::clone;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
    connected_to: Vec<JunctionBox>
}

fn main() {
    println!("Hello, world!");
}

// fn process(junction_boxes: Vec<JunctionBox>) -> u32 {
    //Order by shortest connection
    // junction_boxes.sort_by(|a,b| (a.x + a.y + a.z).cmp(&(b.x + b.y + b.z)));

    // for _ in 0..1000 {
        //Connect

    // }; 
// }

fn connect_shortest(mut junction_boxes: Vec<JunctionBox>) -> Vec<JunctionBox> {
    junction_boxes.sort_by(|a,b| (a.x + a.y + a.z).cmp(&(b.x + b.y + b.z)));

    // junction_boxes.iter().for_each(|jb| {
    //     // let distance = junction_boxes.clone();
    //     junction_boxes.iter().for_each(|jb2| {
    //         // distance.sort_by();

    //         if jb != jb2 {
    //             //Find shortest for jb
    //             if jb < jb2 {
                    
    //             }
                
    //         }
    //     });
    // });

    junction_boxes
}

#[test]
fn test() {
    let junction_boxes: Vec<JunctionBox> = vec![
        JunctionBox { x: 162, y: 817, z: 812, connected_to: vec![] },
        JunctionBox { x: 57, y: 618, z: 57, connected_to: vec![] },
        JunctionBox { x: 906, y: 360, z: 560, connected_to: vec![] },
        JunctionBox { x: 592, y: 479, z: 940, connected_to: vec![] },
        JunctionBox { x: 352, y: 342, z: 300, connected_to: vec![] },
        JunctionBox { x: 466, y: 668, z: 158, connected_to: vec![] },
        JunctionBox { x: 542, y: 29, z: 236, connected_to: vec![] },
        JunctionBox { x: 431, y: 825, z: 988, connected_to: vec![] },
        JunctionBox { x: 739, y: 650, z: 466, connected_to: vec![] },
        JunctionBox { x: 52, y: 470, z: 668, connected_to: vec![] },
        JunctionBox { x: 216, y: 146, z: 977, connected_to: vec![] },
        JunctionBox { x: 819, y: 987, z: 18, connected_to: vec![] },
        JunctionBox { x: 117, y: 168, z: 530, connected_to: vec![] },
        JunctionBox { x: 805, y: 96, z: 715, connected_to: vec![] },
        JunctionBox { x: 346, y: 949, z: 466, connected_to: vec![] },
        JunctionBox { x: 970, y: 615, z: 88, connected_to: vec![] },
        JunctionBox { x: 941, y: 993, z: 340, connected_to: vec![] },
        JunctionBox { x: 862, y: 61, z: 35, connected_to: vec![] },
        JunctionBox { x: 984, y: 92, z: 344, connected_to: vec![] },
        JunctionBox { x: 425, y: 690, z: 689, connected_to: vec![] },
    ];

    let t = connect_shortest(junction_boxes);

    println!("{t:?}");

    assert_eq!(true, false);
    // assert_eq!(process(junction_boxes), 40);
}