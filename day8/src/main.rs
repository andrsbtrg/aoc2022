fn main() {
    // test();
    let input = std::fs::read_to_string("./input.txt").unwrap();
    part_2(&input);
}

fn part_2(input: &str) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let grid_raw: Vec<usize> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let grid: Vec<_> = grid_raw.chunks(width).collect();

    assert!(grid.len() == height);

    let mut visibility: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width]).collect();

    // Loop left to right
    (1..height - 1).for_each(|i| {
        let mut max_height = grid[i][0];
        (1..width - 1).for_each(|j| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });
    // Loop right to left
    (1..height - 1).for_each(|i| {
        let mut max_height = grid[i][width - 1];
        (1..width - 1).rev().for_each(|j| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    // Loop top to bottom
    (1..width - 1).for_each(|j| {
        let mut max_height = grid[0][j];
        (1..height - 1).for_each(|i| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    // Loop bottom to top
    (1..width - 1).for_each(|j| {
        let mut max_height = grid[height - 1][j];
        (1..height - 1).rev().for_each(|i| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    let views: Vec<Vec<usize>> = (1..height - 1)
        .map(|i| {
            (1..width - 1)
                .map(|j| {
                    if visibility[i][j] == true {
                        // check up
                        let mut view_up = 1;
                        let mut up = i - 1;
                        while up > 0 {
                            if grid[up][j] >= grid[i][j] {
                                break;
                            } else {
                                view_up += 1;
                                up -= 1
                            }
                        }
                        let mut view_down = 1;
                        let mut down = i + 1;
                        while down < height - 1 {
                            if grid[down][j] >= grid[i][j] {
                                break;
                            } else {
                                down += 1;
                            }
                        }
                        view_down += 1;
                        let mut view_left = 1;
                        let mut left = j - 1;
                        while left > 0 {
                            if grid[i][left] >= grid[i][j] {
                                break;
                            } else {
                                view_left += 1;
                                left -= 1;
                            }
                        }
                        let mut view_right = 1;
                        let mut right = j + 1;
                        while right < width - 1 {
                            if grid[i][right] >= grid[i][j] {
                                break;
                            } else {
                                view_right += 1;
                                right += 1;
                            }
                        }
                        view_right * view_left * view_up * view_down
                    } else {
                        0
                    }
                })
                .collect()
        })
        .collect();
    let max = views
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Max is : {}", max);
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

    let mut visibility: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width]).collect();

    // Loop left to right
    (1..height - 1).for_each(|i| {
        let mut max_height = grid[i][0];
        (1..width - 1).for_each(|j| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });
    // Loop right to left
    (1..height - 1).for_each(|i| {
        let mut max_height = grid[i][width - 1];
        (1..width - 1).rev().for_each(|j| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    // Loop top to bottom
    (1..width - 1).for_each(|j| {
        let mut max_height = grid[0][j];
        (1..height - 1).for_each(|i| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    // Loop bottom to top
    (1..width - 1).for_each(|j| {
        let mut max_height = grid[height - 1][j];
        (1..height - 1).rev().for_each(|i| {
            if grid[i][j] > max_height {
                visibility[i][j] = visibility[i][j] || true;
                max_height = grid[i][j];
            }
        });
    });

    let visible: usize = visibility
        .iter()
        .map(|row| {
            row.iter()
                .map(|b| match b {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    dbg!(visibility);
    println!("visible inside: {}", visible);
    let visible_outside = (width - 1) * 2 + (height - 1) * 2;
    println!("total visible = {}", visible + visible_outside);
}

fn test() {
    let input = "30373
25512
65332
33549
35390"
        .to_string();
    part_2(&input);
}
