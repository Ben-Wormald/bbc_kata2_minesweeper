use std::fs;
use std::char;

const MINE: char = '*';

fn main() {
    let input = fs::read_to_string("./input").expect("oh no!");
    let grid = parse_input(&input);
    let output = get_ouput(grid);
    print_output(output);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect()
}

fn get_ouput(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().enumerate().map(|(y, line)| {
        line.iter().enumerate().map(|(x, &character)| {
            match character {
                MINE => MINE,
                _ => count_adj_mines(x, y, &grid),
            }
        }).collect()
    }).collect()
}

fn count_adj_mines(x: usize, y: usize, grid: &Vec<Vec<char>>) -> char {
    let w = grid[0].len();
    let h = grid.len();

    let min_x = if x == 0 { 0 } else { x - 1 };
    let max_x = if x == w - 1 { w } else { x + 2 };
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_y = if y == h - 1 { h } else { y + 2 };

    let mut sum = 0;
    for i in min_x..max_x {
        for j in min_y..max_y {
            sum += match grid[j][i] {
                MINE => 1,
                _ => 0,
            }
        }
    }
    char::from_digit(sum, 10).unwrap()
}

fn print_output(grid: Vec<Vec<char>>) -> () {
    grid.iter().for_each(|line| {
        line.iter().for_each(|character| {
            print!("{}", character);
        });
        print!("\n");
    });
}


#[test]
fn counts_no_mines() {
    let test_grid = vec![
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '.'],
    ];
    let actual = count_adj_mines(1, 1, &test_grid);
    assert_eq!(actual, '0');
}

#[test]
fn counts_some_mines() {
    let test_grid = vec![
        vec!['*', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '.', '*'],
    ];
    let actual = count_adj_mines(1, 1, &test_grid);
    assert_eq!(actual, '2');
}

#[test]
fn counts_all_mines() {
    let test_grid = vec![
        vec!['*', '*', '*'],
        vec!['*', '.', '*'],
        vec!['*', '*', '*'],
    ];
    let actual = count_adj_mines(1, 1, &test_grid);
    assert_eq!(actual, '8');
}

#[test]
fn handles_corner() {
    let test_grid = vec![
        vec!['.', '.', '.'],
        vec!['.', '*', '*'],
        vec!['.', '*', '.'],
    ];
    let actual = count_adj_mines(2, 2, &test_grid);
    assert_eq!(actual, '3');
}
