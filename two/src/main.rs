use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

const WIN: u64 = 6;
const DRAW: u64 = 3;
const LOSE: u64 = 0;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strat {
    Lose,
    Draw,
    Win,
}

impl FromStr for Strat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "X" => Strat::Lose,
            "Y" => Strat::Draw,
            "Z" => Strat::Win,
            _ => return Err(anyhow!("Invalid character: {s}")),
        };
        Ok(out)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn wins_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
    fn loses_against(&self) -> RPS {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

impl From<RPS> for u64 {
    fn from(s: RPS) -> u64 {
        match s {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl FromStr for RPS {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => return Err(anyhow!("Invalid character: {s}")),
        };
        Ok(out)
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let rps = read_rps(&opts.input)?;
    let mut rps_score = 0;
    for (first, second) in rps {
        rps_score += calc_rps(first, second);
    }
    println!("RPS Score: {rps_score}");
    let mut strat_score = 0;
    let strat = read_strat(&opts.input)?;
    for (first, second) in strat {
        strat_score += calc_strat(first, second);
    }
    println!("Strat Score: {strat_score}");
    Ok(())
}

fn calc_strat(first: RPS, second: Strat) -> u64 {
    match second {
        Strat::Draw => <RPS as Into<u64>>::into(first) + DRAW,
        Strat::Lose => <RPS as Into<u64>>::into(first.wins_against()) + LOSE,
        Strat::Win => <RPS as Into<u64>>::into(first.loses_against()) + WIN,
    }
}

fn calc_rps(first: RPS, second: RPS) -> u64 {
    if first == second {
        return <RPS as Into<u64>>::into(second) + DRAW;
    }
    if first == RPS::Rock && second == RPS::Paper
        || first == RPS::Paper && second == RPS::Scissors
        || first == RPS::Scissors && second == RPS::Rock
    {
        return <RPS as Into<u64>>::into(second) + WIN;
    }
    return <RPS as Into<u64>>::into(second) + LOSE;
}

fn read_rps(path: &Path) -> Result<Vec<(RPS, RPS)>> {
    read_impl::<RPS>(path)
}

fn read_strat(path: &Path) -> Result<Vec<(RPS, Strat)>> {
    read_impl::<Strat>(path)
}

fn read_impl<T>(path: &Path) -> Result<Vec<(RPS, T)>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    let mut res = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let [first, second] = &line.split(" ").collect::<Vec<_>>()[..] {
            let firstc = RPS::from_str(first)?;
            let secondc = T::from_str(second).unwrap();
            res.push((firstc, secondc));
        }
    }
    Ok(res)
}
