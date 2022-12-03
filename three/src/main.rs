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
    let rucks = read_input(&opts.input)?;
    let mut score = 0;
    for (first, second) in &rucks {
        let d = diff(first, second);
        score += value(d);
    }
    println!("Score: {score}");
    let mut badge_score = 0;
    for chunk in read_input_no_split(&opts.input)?.chunks(3) {
        if let [first, second, third] = chunk {
            let c = common(first, second, third);
            badge_score += value(c);
        }
    }
    println!("Badge Score: {badge_score}");

    Ok(())
}

fn common(first: &str, second: &str, third: &str) -> char {
    let fset = first.chars().collect::<HashSet<_>>();
    let sset = second.chars().collect::<HashSet<_>>();
    let tset = third.chars().collect::<HashSet<_>>();
    let diff = fset.intersection(&sset).map(|x| *x).collect::<HashSet<_>>();
    *diff.intersection(&tset).next().unwrap()
}

fn diff(first: &str, second: &str) -> char {
    let fset = first.chars().collect::<HashSet<_>>();
    let sset = second.chars().collect::<HashSet<_>>();
    *fset.intersection(&sset).next().unwrap()
}

fn value(c: char) -> u64 {
    match c {
        val @ 'a'..='z' => (val as u64 - 97) + 1,
        val @ 'A'..='Z' => (val as u64 - 65) + 27,
        _ => panic!("Invalid value {c}"),
    }
}

fn read_input_no_split(path: &Path) -> Result<Vec<String>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input.lines().map(|x| x.to_string()).collect())
}

fn read_input(path: &Path) -> Result<Vec<(String, String)>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    let mut res = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        let len = trimmed.len();
        let first = trimmed[0..len / 2].to_string();
        let second = trimmed[len / 2..len].to_string();
        res.push((first, second));
    }
    Ok(res)
}
