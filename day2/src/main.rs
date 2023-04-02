fn main() {
    process2()
}

fn process() {
    use std::fs;
    let data = fs::read_to_string("./input.txt").unwrap();
    // println!("{data}");
    let total_score:u32 = data.lines()
        .map(|play| {
            let mut split = play.split(" ");
            let s: &str = split.next().unwrap_or("");
            let u: &str = split.next().unwrap_or("");
            
            let points:u32 = match (s, u) {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,
                ("B", "X") => 0,
                ("C", "Z") => 3,
                ("C", "X") => 6,
                ("C", "Y") => 0,
                (_, _ ) => 0,
            };
            let shape_points = match u {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            points + shape_points

        })
        .sum();
    println!("{total_score}");
}

fn process2() {
    use std::fs;
    let data = fs::read_to_string("./input.txt").unwrap();
    // println!("{data}");
    let total_score:u32 = data.lines()
        .map(|play| {
            let mut split = play.split(" ");
            let s: &str = split.next().unwrap_or("");
            let u: &str = split.next().unwrap_or("");
            
            let yours:&str = match (s, u) {
                ("A", "X") => "Z",
                ("A", "Y") => "X",
                ("A", "Z") => "Y",
                ("B", "X") => "X",
                ("B", "Y") => "Y",
                ("B", "Z") => "Z",
                ("C", "X") => "Y",
                ("C", "Y") => "Z",
                ("C", "Z") => "X",
                (_, _ ) => "",
            };
            let points = match yours {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };
            let result_points = match u {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => 0,
            };
            points + result_points

        })
        .sum();
    println!("{total_score}");
}