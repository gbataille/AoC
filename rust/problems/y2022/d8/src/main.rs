use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let grid = make_tree_grid(contents);
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

    for line_idx in 0..grid.len() {
        let mut line = get_line(&grid, line_idx);

        // From left
        visible_trees.insert((line_idx, 0));
        let mut cur_height = line[0];
        for (col_idx, tree) in line.iter().enumerate() {
            if col_idx == 0 {
                continue;
            }
            if *tree > cur_height {
                cur_height = *tree;
                visible_trees.insert((line_idx, col_idx));
            }
        }

        // From right
        line.reverse();
        visible_trees.insert((line_idx, line.len() - 1));
        let mut cur_height = line[0];
        for (rev_col_idx, tree) in line.iter().enumerate() {
            if rev_col_idx == 0 {
                continue;
            }
            if *tree > cur_height {
                cur_height = *tree;
                visible_trees.insert((line_idx, line.len() - 1 - rev_col_idx));
            }
        }
    }
    for col_idx in 0..grid[0].len() {
        let mut col = get_column(&grid, col_idx);

        // From top
        visible_trees.insert((0, col_idx));
        let mut cur_height = col[0];
        for (line_idx, tree) in col.iter().enumerate() {
            if line_idx == 0 {
                continue;
            }
            if *tree > cur_height {
                cur_height = *tree;
                visible_trees.insert((line_idx, col_idx));
            }
        }

        // From bottom
        col.reverse();
        visible_trees.insert((col.len() - 1, col_idx));
        let mut cur_height = col[0];
        for (rev_line_idx, tree) in col.iter().enumerate() {
            if rev_line_idx == 0 {
                continue;
            }
            if *tree > cur_height {
                cur_height = *tree;
                visible_trees.insert((col.len() - 1 - rev_line_idx, col_idx));
            }
        }
    }

    println!("Trees visible: {}", visible_trees.len());
}

fn part2(contents: &String) {
    let grid = make_tree_grid(contents);
    let mut best_score = 0;

    for line_idx in 0..grid.len() - 1 {
        for col_idx in 0..grid[0].len() - 1 {
            let score = score_for(&grid, line_idx, col_idx);

            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("2520 is too low");
    println!("Best score {}", best_score);
}

fn score_for(grid: &Vec<Vec<u32>>, line_idx: usize, col_idx: usize) -> u32 {
    let mut ls = 0;
    let mut rs = 0;
    let mut us = 0;
    let mut ds = 0;
    let view_height = grid[line_idx][col_idx];

    // To left
    if col_idx > 0 {
        for ci in 1..=col_idx {
            let tree_height = grid[line_idx][col_idx - ci];
            ls += 1;
            if tree_height >= view_height {
                break;
            }
        }
    }
    // To right
    if col_idx < grid[0].len() - 1 {
        for ci in col_idx + 1..grid[0].len() {
            let tree_height = grid[line_idx][ci];
            rs += 1;
            if tree_height >= view_height {
                break;
            }
        }
    }
    // To up
    if line_idx > 0 {
        for li in 1..=line_idx {
            let tree_height = grid[line_idx - li][col_idx];
            us += 1;
            if tree_height >= view_height {
                break;
            }
        }
    }
    // To down
    if line_idx < grid.len() - 1 {
        for li in line_idx + 1..grid.len() {
            let tree_height = grid[li][col_idx];
            ds += 1;
            if tree_height >= view_height {
                break;
            }
        }
    }

    let score = ls * rs * us * ds;
    println!(
        "Score for {}, {} is {} * {} * {} * {} = {}",
        line_idx, col_idx, us, ls, rs, ds, score
    );

    score
}

fn make_tree_grid(contents: &String) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for line in contents.lines() {
        let mut line_vec = Vec::new();
        for tree in line.chars() {
            line_vec.push(tree.to_digit(10).unwrap());
        }

        grid.push(line_vec);
    }

    grid
}

fn get_line(grid: &Vec<Vec<u32>>, line_idx: usize) -> Vec<u32> {
    grid[line_idx].clone()
}

fn get_column(grid: &Vec<Vec<u32>>, column_idx: usize) -> Vec<u32> {
    let mut col: Vec<u32> = Vec::new();
    for line in grid.iter() {
        col.push(line[column_idx]);
    }
    col
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "30373
25512
65332
33549
35390",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }

    #[test]
    fn test_grid() {
        let test_contents = String::from_str(
            "30373
25512
65332
33549
35390",
        )
        .unwrap();

        let grid = make_tree_grid(&test_contents);
        score_for(&grid, 2, 0);
    }
}
