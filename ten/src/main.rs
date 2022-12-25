use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::fmt;
use std::fs::File as StdFile;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let input = read_input(&opts.input)?;
    let mut comp = Computer::new();
    for op in input {
        comp.do_op(op);
    }
    println!("Signal: {}", comp.result);
    eprintln!("{}", comp.screen);
    Ok(())
}

#[derive(Debug)]
struct Screen {
    x: usize,
    y: usize,
    lines: [[char; 40]; 6],
}

impl Screen {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            lines: [[' '; 40]; 6],
        }
    }
    fn inc_x(&mut self, sprite_center: isize) {
        if ((sprite_center - 1)..=(sprite_center + 1)).contains(&(self.x as isize)) {
            self.lines[self.y][self.x] = '#';
        } else {
            self.lines[self.y][self.x] = '.';
        }
        self.x += 1;
        if self.x == 40 {
            self.x = 0;
            self.y += 1;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.lines {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug)]
struct Computer {
    cycle: usize,
    x: isize,
    result: usize,
    screen: Screen,
}

impl Computer {
    fn new() -> Self {
        Self {
            cycle: 1,
            x: 1,
            result: 0,
            screen: Screen::new(),
        }
    }

    fn do_op(&mut self, op: Op) {
        match op {
            Op::Noop => {
                self.inc_cycle();
            }
            Op::AddX(val) => {
                for _ in 0..=1 {
                    self.inc_cycle();
                }
                self.inc_x(val);
            }
        }
    }
    fn inc_x(&mut self, val: isize) {
        self.x += val;
    }
    fn inc_cycle(&mut self) {
        self.check_cycle();
        self.screen.inc_x(self.x);
        self.cycle += 1;
    }

    fn check_cycle(&mut self) {
        match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let add = self.cycle * self.x as usize;
                self.result += add;
                eprintln!(
                    "hit cycle {}, adding {} * {} = {add} -- result: {}",
                    self.cycle, self.cycle, self.x, self.result
                );
            }
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    AddX(isize),
    Noop,
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "noop" => Ok(Self::Noop),
            other => {
                if let [op, val] = &other.split(' ').collect::<Vec<_>>()[..] {
                    match *op {
                        "addx" => Ok(Self::AddX(val.parse::<isize>().unwrap())),
                        _ => Err(anyhow!("invalid op: {op}")),
                    }
                } else {
                    Err(anyhow!("invalid op: {other}"))
                }
            }
        }
    }
}

fn read_input(path: &Path) -> Result<Vec<Op>> {
    let mut input = String::new();
    let mut f = StdFile::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input.lines().map(|x| Op::from_str(x).unwrap()).collect())
}
