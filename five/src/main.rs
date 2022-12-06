use anyhow::Result;
use clap::Parser;
use std::fs::File;
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
    let (s, o) = read_input(&opts.input)?;
    let mut stacks = parse_stacks(s.clone());
    let ops = parse_ops(o);
    for op in ops.clone() {
        for _ in 0..op.num {
            if let Some(c) = stacks.pop(op.from) {
                stacks.push(op.to, c);
            }
        }
    }
    println!("Tops: {}", stacks.tops());
    let mut stacks = parse_stacks(s);
    for op in ops {
        let cs = stacks.pop_mult(op.from, op.num);
        stacks.push_mult(op.to, cs);
    }
    println!("Tops Mult: {}", stacks.tops());
    Ok(())
}

#[derive(Debug)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn new(len: usize) -> Self {
        let mut inner: Vec<Vec<char>> = Vec::new();
        for _ in 0..len {
            inner.push(Vec::new());
        }
        Self(inner)
    }

    fn push(&mut self, index: usize, c: char) {
        self.0[index - 1].push(c)
    }

    fn push_mult(&mut self, index: usize, cs: Vec<char>) {
        self.0[index - 1].extend(cs)
    }

    fn pop(&mut self, index: usize) -> Option<char> {
        self.0[index - 1].pop()
    }

    fn pop_mult(&mut self, index: usize, num: usize) -> Vec<char> {
        let mut res = Vec::new();
        for _ in 0..num {
            if let Some(c) = self.0[index - 1].pop() {
                res.push(c)
            }
        }
        res.into_iter().rev().collect()
    }

    fn tops(&self) -> String {
        self.0.iter().map(|x| x.last().unwrap()).collect()
    }
}

#[derive(Debug, Clone)]
struct Op {
    num: usize,
    from: usize,
    to: usize,
}

fn parse_ops(s: Vec<String>) -> Vec<Op> {
    let mut res = Vec::new();
    for line in s {
        if let [num, from, to] = &line
            .chars()
            .filter(|c| c.is_numeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()[..]
        {
            let num = num.parse::<usize>().unwrap();
            let from = from.parse::<usize>().unwrap();
            let to = to.parse::<usize>().unwrap();
            res.push(Op { num, from, to })
        }
    }
    res
}

fn parse_stacks(s: Vec<String>) -> Stacks {
    let stack_num: usize = s
        .iter()
        .map(|x| x.chars().filter(|y| y.is_numeric()).count())
        .max()
        .unwrap();
    let mut stacks = Stacks::new(stack_num);
    for line in s.into_iter().rev().skip(1) {
        let mut index = 1;
        for chunk in line.chars().collect::<Vec<char>>().chunks(4) {
            for c in chunk {
                if c.is_alphabetic() {
                    stacks.push(index, *c);
                }
            }
            index += 1
        }
    }
    stacks
}

fn read_input(path: &Path) -> Result<(Vec<String>, Vec<String>)> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    let stacks = input
        .as_str()
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let ops = input
        .as_str()
        .lines()
        .skip(stacks.len() + 1)
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok((stacks, ops))
}
