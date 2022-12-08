use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::fs::File as StdFile;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let input = read_input(&opts.input)?;
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for (index, row) in input.iter().enumerate() {
        find_vis(row, index, &mut visible, false);
    }
    let input_ref: Vec<&[i32]> = input.iter().map(|x| &x[..]).collect();
    for index in 0..input[0].len() {
        let row = get_col(&input_ref, index);
        find_vis(&row, index, &mut visible, true);
    }
    let vis_len = visible.len();
    println!("Visible total: {vis_len}");

    let mut max_sc = 0;
    for x in 0..input_ref.len() {
        for y in 0..input_ref[x].len() {
            let sc = senic_score(&input_ref, x, y);
            if sc > max_sc {
                max_sc = sc;
            }
        }
    }
    println!("Maximum scenic score: {max_sc}");
    Ok(())
}

fn find_vis(trees: &[i32], index: usize, visible: &mut HashSet<(usize, usize)>, cols: bool) {
    let mut max_height: i32 = -1;
    for (i, tree) in trees.iter().enumerate() {
        if *tree > max_height {
            max_height = *tree;
            if cols {
                visible.insert((i, index));
            } else {
                visible.insert((index, i));
            }
        }
    }
    max_height = -1;
    for (rev_i, tree) in trees.iter().rev().enumerate() {
        let i = (trees.len() - 1) - rev_i;
        if *tree > max_height {
            max_height = *tree;
            if cols {
                visible.insert((i, index));
            } else {
                visible.insert((index, i));
            }
        }
    }
}

fn senic_score(trees: &[&[i32]], x: usize, y: usize) -> usize {
    let house = trees[x][y];
    let mut down = 0;
    for i in (x + 1)..trees.len() {
        down += 1;
        if trees[i][y] >= house {
            break;
        }
    }

    let mut up = 0;
    for i in (0..x).rev() {
        up += 1;
        if trees[i][y] >= house {
            break;
        }
    }

    let mut left = 0;
    for i in (0..y).rev() {
        left += 1;
        if trees[x][i] >= house {
            break;
        }
    }

    let mut right = 0;
    for i in (y + 1)..trees[x].len() {
        right += 1;
        if trees[x][i] >= house {
            break;
        }
    }
    up * down * left * right
}

fn get_col(trees: &[&[i32]], index: usize) -> Vec<i32> {
    let mut col = Vec::new();
    for row in trees {
        col.push(row[index]);
    }
    col
}

fn read_input(path: &Path) -> Result<Vec<Vec<i32>>> {
    let mut input = String::new();
    let mut f = StdFile::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect())
}
