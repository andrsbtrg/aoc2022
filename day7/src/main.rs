use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    fs,
    rc::Rc,
};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    process2(&input);
}
/**
.
*/
fn process2(input: &str) {
    let root: Rc<Folder> = Rc::default();

    let mut cwd = Rc::clone(&root);

    input.lines().for_each(|l| {
        // println!("cwd: {}", current_dir);
        let words = l.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dirname => {
                    let new_dir = cwd.subfolder.borrow().get(dirname).unwrap().clone();
                    cwd = new_dir;
                }
            },

            ("dir", folder_name) => {
                let new_dir = Rc::new(Folder::new(folder_name.to_string(), Some(Rc::clone(&cwd))));
                cwd.subfolder
                    .borrow_mut()
                    .insert(folder_name.to_string(), new_dir);
            }
            (size, _filename) => {
                let file_size: usize = size.parse().unwrap();
                *cwd.size.borrow_mut() += file_size;
            }
        }
    });

    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let needed = 30000000 - free_space;

    let mut best = usize::MAX;

    println!("free : {}", free_space);
    println!("needed: {}", needed);

    let mut to_visit = vec![root];

    while let Some(dir) = to_visit.pop() {
        for d in dir.subfolder.borrow().values() {
            to_visit.push(Rc::clone(d));
        }
        let size = dir.get_size();
        if size >= needed {
            best = best.min(size);
        }
    }
    println!("best = {}", best);
}
fn process1(input: &str) {
    let root: Rc<Folder> = Rc::default();

    let mut cwd = Rc::clone(&root);

    input.lines().for_each(|l| {
        // println!("cwd: {}", current_dir);
        let words = l.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dirname => {
                    let new_dir = cwd.subfolder.borrow().get(dirname).unwrap().clone();
                    cwd = new_dir;
                }
            },

            ("dir", folder_name) => {
                let new_dir = Rc::new(Folder::new(folder_name.to_string(), Some(Rc::clone(&cwd))));
                cwd.subfolder
                    .borrow_mut()
                    .insert(folder_name.to_string(), new_dir);
            }
            (size, _filename) => {
                let file_size: usize = size.parse().unwrap();
                *cwd.size.borrow_mut() += file_size;
            }
        }
    });

    let mut total: usize = 0;
    let mut to_visit = vec![root];

    while let Some(dir) = to_visit.pop() {
        for d in dir.subfolder.borrow().values() {
            to_visit.push(Rc::clone(d));
        }
        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }
    println!("total = {}", total);
}

#[derive(Default)]
struct Folder {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Folder>>,
    subfolder: RefCell<HashMap<String, Rc<Folder>>>,
}
impl Folder {
    fn new(name: String, parent: Option<Rc<Folder>>) -> Self {
        Folder {
            name,
            size: RefCell::new(0),
            parent,
            subfolder: RefCell::new(HashMap::new()),
        }
    }
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subfolder
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

#[test]
fn test() {
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    process1(INPUT);
}
