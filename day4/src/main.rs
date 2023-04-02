use std::fs;
fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("error");

    let total = process2(input);
    
    println!("{total}");

        
}

fn process(input: String) ->u32 {
    let total:Vec<u32>= input.lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            let (a, b) = left.split_once("-")
                .unwrap();
            let a_u: u32 = a.parse().unwrap();
            let b_u: u32 = b.parse().unwrap();

            let (c,d) = right.split_once("-").unwrap();
            let c_u: u32 = c.parse().unwrap();
            let d_u: u32 = d.parse().unwrap();

            let a_contains_b = match (a_u..=b_u).contains(&c_u)  {
                true => {
                    match (a_u..=b_u).contains(&d_u) {
                        true => true,
                        false => false,
                }},
                false => false };

            let b_contains_a = {
                match (c_u..=d_u).contains(&a_u) {
                    true => match (c_u..=d_u).contains(&b_u) {
                        true => true,
                        false => false,
                    },
                    false => false
            }};

            match b_contains_a || a_contains_b {
                true => 1,
                false => 0,
            }
            }).collect();
        // dbg!(&total);
        total.iter().sum()
        // total

}

fn process2(input: String) ->u32 {
    let total:Vec<u32>= input.lines()
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap();

            let (a, b) = left.split_once("-")
                .unwrap();
            let a_u: u32 = a.parse().unwrap();
            let b_u: u32 = b.parse().unwrap();

            let (c,d) = right.split_once("-").unwrap();
            let c_u: u32 = c.parse().unwrap();
            let d_u: u32 = d.parse().unwrap();

            let a_contains_b = match (a_u..=b_u).contains(&c_u)  {
                true => { true
                },
                false => false };

            let b_contains_a = {
                match (c_u..=d_u).contains(&a_u) {
                    true => true,
                    false => false
            }};

            match b_contains_a || a_contains_b {
                true => 1,
                false => 0,
            }
            }).collect();
        // dbg!(&total);
        total.iter().sum()
        // total

}

#[test]
fn test () {
    let input = std::fs::read_to_string("./test.txt").unwrap();

    let total = process(input);
    println!("total={total}");
    // assert!(total==2);
}

#[test]
fn test2 () {
    let contains = (4..=6).contains(&6);
    assert!(contains);
}

