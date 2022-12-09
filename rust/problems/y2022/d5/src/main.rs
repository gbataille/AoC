use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    part1(&lines);
    part2(&lines);
}

#[derive(Debug)]
struct Instr {
    qty: i32,
    from: usize,
    to: usize,
}

fn part1(lines: &Vec<&str>) {
    let (crate_lines, instr_lines) = parse_input(lines);
    let mut stacks = parse_crate_lines(crate_lines);
    let instrs = parse_instr_lines(instr_lines);

    for instr in instrs {
        for _step in 0..instr.qty {
            let c = stacks.get_mut(&instr.from).unwrap().pop().unwrap();
            stacks.get_mut(&instr.to).unwrap().push(c);
        }
    }

    for i in 1..=stacks.len() {
        let stack = stacks.get(&i).unwrap();
        print!("{}", stack[stack.len() - 1]);
    }
    println!("");
}

fn print_stacks(stacks: &HashMap<usize, Vec<char>>) {
    for (k, v) in stacks {
        println!("Stack {k}: {v:?}");
    }
    println!("==============");
}

fn parse_input<'a, 'b>(lines: &'a Vec<&'b str>) -> (&'a [&'b str], &'a [&'b str]) {
    let mut crate_lines_end = 0;
    for line in lines.iter() {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        } else {
            crate_lines_end += 1;
        }
    }
    (
        &lines.as_slice()[..crate_lines_end],
        &lines.as_slice()[crate_lines_end + 2..],
    )
}

fn parse_crate_lines(crate_lines: &[&str]) -> HashMap<usize, Vec<char>> {
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let ignored_chars = ['[', ']', ' '];

    let base = &crate_lines[0];
    let nb_stacks = (base.len() + 1) / 4;
    for s in 1..=nb_stacks {
        stacks.insert(s, Vec::new());
    }

    for idx in (0..(crate_lines.len())).rev() {
        let line = crate_lines[idx];
        for (i, c) in line.chars().enumerate() {
            if ignored_chars.contains(&c) {
                continue;
            }
            let stack_nb = (i - 1) / 4 + 1;
            stacks.get_mut(&stack_nb).unwrap().push(c);
        }
    }
    stacks
}

fn parse_instr_lines(instr_lines: &[&str]) -> Vec<Instr> {
    let mut instrs = Vec::new();

    for line in instr_lines {
        if line.len() == 0 {
            continue;
        }
        let tokens: Vec<&str> = line.split(" ").collect();
        instrs.push(Instr {
            qty: FromStr::from_str(tokens[1]).unwrap(),
            from: FromStr::from_str(tokens[3]).unwrap(),
            to: FromStr::from_str(tokens[5]).unwrap(),
        });
    }

    instrs
}

fn part2(lines: &Vec<&str>) {
    let (crate_lines, instr_lines) = parse_input(lines);
    let mut stacks = parse_crate_lines(crate_lines);
    let instrs = parse_instr_lines(instr_lines);

    for instr in instrs {
        let mut buffer = Vec::new();
        for _step in 0..instr.qty {
            let c = stacks.get_mut(&instr.from).unwrap().pop().unwrap();
            buffer.push(c);
        }
        for _step in 0..instr.qty {
            let c = buffer.pop().unwrap();
            stacks.get_mut(&instr.to).unwrap().push(c);
        }
    }

    for i in 1..=stacks.len() {
        let stack = stacks.get(&i).unwrap();
        print!("{}", stack[stack.len() - 1]);
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let lines: Vec<&str> = contents.split("\n").collect();
        part1(&lines);
    }

    #[test]
    fn test_part2() {
        let contents = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let lines: Vec<&str> = contents.split("\n").collect();
        part2(&lines);
    }
}
