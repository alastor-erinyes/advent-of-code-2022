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
    let mut elves = read_elves(&opts.input)?;
    let max = elves.iter().max().unwrap();
    println!("Max: {max}");
    elves.sort();
    let sum: u64 = elves.into_iter().rev().take(3).sum();
    println!("Sum: {sum}");
    Ok(())
}

fn read_elves(path: &Path) -> Result<Vec<u64>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    let mut elf = 0;
    let mut elves = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            elves.push(elf.clone());
            elf = 0;
        } else {
            elf += line.parse::<u64>().unwrap_or_else(|err| {
                panic!("invalid u64: {err:?} {line}");
            });
        }
    }
    Ok(elves)
}
