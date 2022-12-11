use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, Default)]
struct File {
    size: usize,
}

#[derive(Debug, Default)]
struct Folder<'a> {
    name: &'a str,
    folders: RefCell<HashMap<&'a str, Rc<Folder<'a>>>>,
    files: RefCell<HashMap<&'a str, File>>,
    parent: Weak<Folder<'a>>,
    size: RefCell<usize>,
}

impl<'a> Folder<'a> {
    fn build_root() -> Self {
        Folder::with_name_and_parent(&"/", Weak::new())
    }

    fn with_name_and_parent(name: &'a str, parent: Weak<Folder<'a>>) -> Self {
        Folder {
            name,
            folders: RefCell::new(HashMap::new()),
            files: RefCell::new(HashMap::new()),
            parent,
            size: RefCell::new(0),
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let root = parse_fs_output(contents);
    dbg!(root.clone());
    println!("45872605 too high");
    println!("{}", bounded_part1_size(root.clone()));
}

fn part2(contents: &String) {
    let root = parse_fs_output(contents);
    let used_space = total_size(root.clone());
    println!("Used space {}", used_space);
    let needed = 30000000 - (70000000 - used_space);
    println!("Needed {}", needed);
    let f = part2_smallest_sub_dir_bigger_than_needed(root.clone(), needed).unwrap();
    println!("Folder {}, of size {}", f.name, total_size(f.clone()));
}

fn part2_smallest_sub_dir_bigger_than_needed(
    folder: Rc<Folder>,
    needed: usize,
) -> Option<Rc<Folder>> {
    let mut min_folder = folder.clone();
    if total_size(min_folder.clone()) < needed {
        return None;
    } else {
        for (_key, child) in (*folder.folders.borrow()).iter() {
            match part2_smallest_sub_dir_bigger_than_needed(child.clone(), needed) {
                None => continue,
                Some(small_child) => {
                    if total_size(small_child.clone()) < total_size(min_folder.clone()) {
                        min_folder = small_child.clone();
                    }
                }
            }
        }
    }

    return Some(min_folder);
}

fn bounded_part1_size(folder: Rc<Folder>) -> usize {
    let mut size = 0;
    for (_key, child) in (*folder.folders.borrow()).iter() {
        size += bounded_part1_size(child.clone())
    }
    let fsize = total_size(folder);
    if fsize < 100000 {
        size += fsize;
    }
    size
}

fn total_size(folder: Rc<Folder>) -> usize {
    let mut size = 0;
    for (_key, child) in (*folder.folders.borrow()).iter() {
        size += total_size(child.clone());
    }
    for (_key, file) in (*folder.files.borrow()).iter() {
        size += file.size;
    }
    println!("Tot size of {}: {}", folder.name, size);
    size
}

fn parse_fs_output(data: &String) -> Rc<Folder> {
    let fs_root = Rc::new(Folder::build_root());
    let mut cur_folder = fs_root.clone();
    for line in data.split("\n") {
        cur_folder = interpret_line(line, cur_folder);
    }

    return fs_root;
}

fn find_root<'a>(folder: Rc<Folder<'a>>) -> Rc<Folder<'a>> {
    match folder.parent.upgrade() {
        Some(parent) => find_root(parent.clone()),
        None => folder.clone(),
    }
}

fn interpret_line<'a>(line: &'a str, current_folder: Rc<Folder<'a>>) -> Rc<Folder<'a>> {
    if line.len() == 0 {
        return current_folder;
    }

    let tokens = line.split(" ").collect::<Vec<&str>>();

    match (tokens[0], tokens[1]) {
        ("$", "cd") => match tokens[2] {
            "/" => find_root(current_folder),
            ".." => current_folder.parent.upgrade().unwrap(),
            dirname => move_down_to(current_folder.clone(), dirname),
        },
        ("$", "ls") => current_folder,
        ("dir", dirname) => {
            add_child_folder(current_folder.clone(), dirname);
            current_folder
        }
        (size, filename) => {
            add_child_file(
                current_folder.clone(),
                filename,
                size.parse::<usize>().unwrap(),
            );
            current_folder
        }
    }
}

fn add_child_file<'a>(folder: Rc<Folder<'a>>, child: &'a str, size: usize) -> Rc<Folder<'a>> {
    *folder.size.borrow_mut() += size;
    folder.files.borrow_mut().insert(child, File { size });
    folder
}

fn add_child_folder<'a>(folder: Rc<Folder<'a>>, child: &'a str) -> Rc<Folder<'a>> {
    folder.folders.borrow_mut().insert(
        child,
        Rc::new(Folder::with_name_and_parent(child, Rc::downgrade(&folder))),
    );
    folder
}

fn move_down_to<'a>(folder: Rc<Folder<'a>>, child_name: &'a str) -> Rc<Folder<'a>> {
    folder.folders.borrow().get(child_name).unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder() {
        let root = Rc::new(Folder::build_root());
        let sub = Rc::new(Folder::with_name_and_parent("sub", Rc::downgrade(&root)));
        let subsub = Rc::new(Folder::with_name_and_parent("subsub", Rc::downgrade(&sub)));
        let subsubsub = Rc::new(Folder::with_name_and_parent(
            "subsubsub",
            Rc::downgrade(&subsub),
        ));

        assert_eq!(find_root(subsubsub.clone()).name, root.name);
        dbg!(find_root(sub.clone()));
    }

    #[test]
    fn test_add_child_file() {
        let root = Rc::new(Folder::build_root());
        add_child_file(root.clone(), &"foo", 123);
        add_child_file(root.clone(), &"bar", 876);

        assert_eq!(*root.size.borrow(), 999);
        dbg!(root);
    }

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "$ cd /
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
7214296 k",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
