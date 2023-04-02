use regex::Regex;


fn main() {
    let re = Regex::new(r"(\d+)").unwrap();
    let crates: Vec<Vec<&str>> = vec![
        vec!["Z", "T", "F", "R", "W", "J", "G"], 
        vec!["G", "W", "M"], 
        vec!["J", "N", "H", "G"], 
        vec!["J", "R", "C", "N", "W"], 
        vec!["W", "F", "S", "B", "G", "Q", "V", "M"], 
        vec!["S", "R", "T", "D", "V", "W", "C"], 
        vec!["H", "B", "N", "C", "D", "Z", "G", "V"], 
        vec!["S", "J", "N", "M", "G", "C"], 
        vec!["G", "P", "N", "W", "C", "J", "D", "L"], 
        ];


    let input = std::fs::read_to_string("./input.txt").unwrap();

    process2(input, re, crates);


    println!("");
}

fn process1(input: String, re: Regex, mut crates: Vec<Vec<&str>>) {

    input.lines().for_each(|line| {

        let matches = re.captures_iter(line);
        
        let mut numbers: Vec<usize> = Vec::new();
    
        matches.into_iter().for_each(|m| {
            let parsed:usize = m[0].parse().unwrap();
            // println!("{parsed}");
            numbers.push(parsed);
    
        });
    
        let pat = Pattern::new(numbers);
    
        // dbg!(pat);

        for _ in 0..pat.number {
            let last = crates.get_mut(pat.from - 1).unwrap().pop().unwrap();

            crates.get_mut(pat.to -1).unwrap().push(last);
        }

    });

    for cr in crates {

        let last1 = *cr.last().unwrap();
        print!("{last1}")
    }
}

fn process2(input: String, re: Regex, mut crates: Vec<Vec<&str>>) {

    input.lines().for_each(|line| {

        let matches = re.captures_iter(line);
        
        let mut numbers: Vec<usize> = Vec::new();
    
        matches.into_iter().for_each(|m| {
            let parsed:usize = m[0].parse().unwrap();
            numbers.push(parsed);
    
        });
    
        let pat = Pattern::new(numbers);
    
        
        let from = crates.get_mut(pat.from - 1).unwrap();
        
        let mut to_move : Vec<&str> = Vec::new();

        (0..pat.number).for_each(|_| {
            to_move.push(from.pop().unwrap());
        });
        to_move.reverse();

        crates.get_mut(pat.to -1).unwrap().append(& mut to_move);
        

    });
    
    for cr in crates {

        let last1 = *cr.last().unwrap();
        print!("{last1}")
    }
}


#[derive(Debug)]
struct Pattern {
    number: usize,
    from: usize,
    to: usize,
}
impl Pattern {
    fn new(numbers: Vec<usize>) -> Self {
        Pattern { number: *numbers.get(0).unwrap(), from: *numbers.get(1).unwrap(), to: *numbers.get(2).unwrap() }
    }
}
