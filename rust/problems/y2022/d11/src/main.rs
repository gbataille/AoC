use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug)]
struct Monkey {
    items: RefCell<VecDeque<u64>>,
    op_symbol: String,
    op_operand: String,
    test_div: u64,
    if_true: u32,
    if_false: u32,
    items_inspected: RefCell<u64>,
}

impl Monkey {
    fn apply_op(&self, item: u64) -> u64 {
        match self.op_symbol.as_str() {
            "+" => match self.op_operand.as_str() {
                "old" => item * 2,
                a => item + a.parse::<u64>().unwrap(),
            },
            "*" => match self.op_operand.as_str() {
                "old" => item * item,
                a => item * a.parse::<u64>().unwrap(),
            },
            _ => item,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let mut monkeys = parse_monkeys(contents);

    for round in 0..20 {
        println!("Round {}", round + 1);
        run_round(&monkeys, 3, std::u64::MAX);
        // dbg!(&monkeys);
        // println!("$$$$$$$$$$$$\n");
    }

    // dbg!(&monkeys);

    monkeys.sort_by_key(|m| *m.items_inspected.borrow());
    monkeys.reverse();

    println!(
        "Monkey business level: {}",
        *monkeys[0].items_inspected.borrow() * *monkeys[1].items_inspected.borrow()
    );
}

fn part2(contents: &String) {
    let mut monkeys = parse_monkeys(contents);

    let mut mod_factor: u64 = 1;
    for monkey in monkeys.iter() {
        mod_factor = mod_factor * monkey.test_div;
    }

    for round in 0..10000 {
        println!("Round {}", round + 1);
        run_round(&monkeys, 1, mod_factor);
        // dbg!(&monkeys);
        // println!("$$$$$$$$$$$$\n");
    }

    // dbg!(&monkeys);

    for monkey in monkeys.iter() {
        println!("items checked: {}", *monkey.items_inspected.borrow());
    }

    monkeys.sort_by_key(|m| *m.items_inspected.borrow());
    monkeys.reverse();

    let monkey_business: u64 =
        *monkeys[0].items_inspected.borrow() * *monkeys[1].items_inspected.borrow();
    println!("Monkey business level: {}", monkey_business);
}

fn parse_monkeys(contents: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut lines = contents.lines();
    loop {
        let _monkey_line = lines.next().unwrap();
        let starting = lines.next().unwrap();
        let operation = lines.next().unwrap();
        let test_str = lines.next().unwrap();
        let test_true = lines.next().unwrap();
        let test_false = lines.next().unwrap();
        let empty = lines.next();

        let items_str: VecDeque<&str> = starting[18..].split(", ").collect();
        let ops: Vec<&str> = operation[23..].split(" ").collect();
        let test_div: u64 = test_str[21..].parse::<u64>().unwrap();
        let if_true: u32 = test_true[29..].parse::<u32>().unwrap();
        let if_false: u32 = test_false[30..].parse::<u32>().unwrap();

        let mut items: VecDeque<u64> = VecDeque::new();
        for i in items_str.iter() {
            items.push_back(i.parse::<u64>().unwrap());
        }
        monkeys.push(Monkey {
            items: RefCell::new(items),
            op_symbol: String::from(ops[0]),
            op_operand: String::from(ops[1]),
            test_div,
            if_true,
            if_false,
            items_inspected: RefCell::new(0),
        });

        // dbg!(monkeys.last().unwrap());

        if empty.is_none() {
            break;
        }
    }

    monkeys
}

fn run_round(monkeys: &Vec<Monkey>, worry_reducer: u64, mod_factor: u64) {
    for monkey in monkeys {
        while monkey.items.borrow().len() > 0 {
            let mut item = monkey.items.borrow_mut().pop_front().unwrap();
            *monkey.items_inspected.borrow_mut() += 1;
            item = monkey.apply_op(item) % mod_factor;
            item = item / worry_reducer;
            let next_monkey: &Monkey;
            if item % monkey.test_div == 0 {
                next_monkey = &monkeys[monkey.if_true as usize];
            } else {
                next_monkey = &monkeys[monkey.if_false as usize];
            }

            next_monkey.items.borrow_mut().push_back(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
