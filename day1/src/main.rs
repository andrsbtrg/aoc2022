use std::fs;

fn main() {
    let data = fs::read_to_string("list.txt").expect("Unable to read file");

    let mut sums: Vec<u32> = Vec::new();

    data.split("\n\n")
        .for_each(|f| {
            let mut sum: u32 = 0;
            f.split("\n")
            .for_each(|s|{
                dbg!(s);
                let d: u32 = s.parse().unwrap_or(0);
                sum = sum +d;
            });
            sums.push(sum);
        }
    );

    sums.sort();
    let top3:u32 = sums[sums.len() - 3 .. sums.len()].iter().sum();


    println!("{top3}");
}
