use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let ans = process(input);
    println!("{}", ans);
}

fn process(input: String) -> i32 {
    for (index, win) in input.into_bytes().windows(14).enumerate() {
        if has_unique_elements(win) {
            let ans = 14 + index as i32;
            println!("{:?}", win);
            println!("{index}");
            return ans;
        }
    }

    return 0;
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();

    assert!(process(input) == 19);
}
