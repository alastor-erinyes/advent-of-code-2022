use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::ops::RangeInclusive;
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
    let mut full_overlap = 0;
    for (e1, e2) in input.clone() {
        if (e1.start() <= e2.start() && e1.end() >= e2.end())
            || (e2.start() <= e1.start() && e2.end() >= e1.end())
        {
            full_overlap += 1;
        }
    }
    println!("Full Overlaps: {full_overlap}");

    let mut partial_overlap = 0;
    for (e1, e2) in input {
        if (e1.contains(e2.start()) || e1.contains(e2.end()))
            || (e2.contains(e1.start()) || e2.contains(e1.end()))
        {
            partial_overlap += 1;
        }
    }
    println!("Partial Overlaps: {partial_overlap}");
    Ok(())
}

fn read_input(path: &Path) -> Result<Vec<(RangeInclusive<u64>, RangeInclusive<u64>)>> {
    let mut input = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut input)?;
    let mut res = Vec::new();
    for line in input.lines() {
        if let [first, second] = &line.split(",").collect::<Vec<_>>()[..] {
            if let [e11, e12] = &first
                .split("-")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()[..]
            {
                if let [e21, e22] = &second
                    .split("-")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    res.push((*e11..=*e12, *e21..=*e22));
                }
            }
        }
    }
    Ok(res)
}
