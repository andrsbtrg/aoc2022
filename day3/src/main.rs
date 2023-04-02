use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    use std::fs;
    let input = fs::read_to_string("./input.txt")
        .expect("unable to read file");
   process2(input);
}
fn process2(input: String) {
    use itertools;

    let repeated_list: Vec<char> = input.lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let mut chars = HashMap::<char, u8>::new();
            // let mut repeated = 'a';
            
            for line in group {
                let mut unique_in_line: Vec<char> = Vec::new();
                // dbg!(line);
                line.chars().for_each(|c| {
                    match unique_in_line.contains(&c) {
                        true => (),
                        false => unique_in_line.push(c),
                    }
                });
                let dbg_str = String::from_iter(unique_in_line.iter());
                dbg!(dbg_str);
                unique_in_line.iter().for_each(|c| {
                    chars.entry(*c).and_modify(|count| *count += 1).or_insert(1);
                });

            }


            let repeated = *chars.iter().find_map(|(key, &val)| {
                if val == 3 {
                    Some(key)
                } else { None }
            }).unwrap();
            println!("____ {repeated}");

            repeated

        }).collect_vec();

        let total = give_score(repeated_list);

        println!("{total}");



}
fn process(input:String) -> u32 {
    
    let character: Vec<char> = input.lines().map(|line| {
        let (left, right) = line.split_at(line.len()/2);
        
        let mut idx: usize = 0;

        left.chars()
            .for_each(|c| {

                let i = right.find(c);
                if i.is_some() {
                    idx = i.unwrap();
                }

            });
        let b = right.as_bytes()[idx];
        let c = b as char;
        
        c
        
    }).collect::<Vec<char>>();

    let tots:u32 = give_score(character);
    
   println!("{tots}"); 
   tots

}

fn give_score(input: Vec<char>) -> u32 {
    input.iter().map(|c| {
        let item = match c.is_uppercase() {
            true => *c as u32 - 65 + 27,
            false => *c as u32 - 96,
        };
        
        item
    }).sum()
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();

    let tots = process(input);

    assert!(tots == 157);
}