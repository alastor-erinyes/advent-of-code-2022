use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
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
    let input = read_input(&opts.input)?;
    let res = first_unique_chars(&input, 4);
    println!("Packet Chars: {res}");
    let res = first_unique_chars(&input, 14);
    println!("Message Chars: {res}");
    Ok(())
}

fn first_unique_chars(input: &Vec<char>, num: usize) -> usize {
    let mut res = 0;
    for i in 0..input.len() - num {
        let hs = input[i..i + num].into_iter().collect::<HashSet<_>>();
        if hs.len() == num {
            res = i + num;
            break;
        }
    }
    res
}

fn read_input(path: &Path) -> Result<Vec<char>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input.chars().collect())
}
