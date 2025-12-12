fn main() {
}

fn process(shapes: Vec<Vec<Vec<bool>>>, regions: Vec<(u32, u32, Vec<u32>)>) {

}

/// Add the shape to the given area
/// 
/// # Arguments
/// shape: Shape to add (true==part of the shape)
/// area: to add the shape (true==already a shape here)
/// 
/// # Returns
/// Err: The shape does not fit
/// Ok: Return the area with the shape
fn add_shape_in_area(shape: Vec<Vec<bool>>, mut area: Vec<Vec<bool>>) -> Result<Vec<Vec<bool>>,()> {
    let shape_size: u32 = shape
        .iter()
        .map(|s| s.iter().filter(|&&b| b == true))
        .count() as u32;

    let area_left: u32 = shape
        .iter()
        .map(|s| s.iter().filter(|&&b| b == false))
        .count() as u32;
    
    if area_left < shape_size {
        return Err(())
    }

    //TODO: Make the algorithm to try all free room in the area left using all rotation each time

    Ok(area)
}

fn test() {
    let shapes: Vec<Vec<Vec<bool>>> = vec![
        // 0:
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, false],
        ],
        // 1:
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![false, true, true],
        ],
        // 2:
        vec![
            vec![false, true, true],
            vec![true, true, true],
            vec![true, true, false],
        ],
        // 3:
        vec![
            vec![true, true, false],
            vec![true, true, true],
            vec![true, true, false],
        ],
        // 4:
        vec![
            vec![true, true, true],
            vec![true, false, false],
            vec![true, true, true],
        ],
        // 5:
        vec![
            vec![true, true, true],
            vec![false, true, false],
            vec![true, true, true],
        ],
    ];

    let regions: Vec<(u32, u32, Vec<u32>)> = vec![
        (4, 4, vec![0, 0, 0, 0, 2, 0]),
        (12, 5, vec![1, 0, 1, 0, 2, 2]),
        (12, 5, vec![1, 0, 1, 0, 3, 2]),
    ];

    process(shapes, regions);
}
