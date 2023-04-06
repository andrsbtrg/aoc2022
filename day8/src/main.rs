fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    part_1(&input);
}

fn part_1(input: &str) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let grid_raw: Vec<usize> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let grid: Vec<_> = grid_raw.chunks(width).collect();

    assert!(grid.len() == height);

    let mut visible: i32 = 0;

    println!("total visible = {}", visible);
}
#[test]
fn test() {
    let input = "30373
25512
65332
33549
35390"
        .to_string();
    part_1(&input);
}
