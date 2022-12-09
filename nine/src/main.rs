use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::collections::VecDeque;
use std::fmt;
use std::fs::File as StdFile;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Parser)]
struct Opts {
    #[clap(help = "Path to input file")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let input = read_input(&opts.input)?;
    let mut map = GrowingMap::new();
    // println!("{map}");
    for movement in input {
        let mut _stdout = std::io::stdout().lock();
        // stdout.write_all(b"\\033[2J");
        // println!("{movement:?}");
        map.move_head(movement);
        // println!("{map}");
        // sleep(Duration::from_secs(1));
    }
    println!("Trail locations: {}", map.calc_trail());
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct GrowingMap {
    map: VecDeque<VecDeque<Cell>>,
    head: Position,
    tail: Position,
}

impl GrowingMap {
    fn new() -> Self {
        let mut map = VecDeque::new();
        let mut row = VecDeque::new();
        row.push_back(Cell::None);
        map.push_back(row);

        Self {
            map,
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        }
    }

    fn calc_trail(&self) -> usize {
        let mut count = 0;
        for row in &self.map {
            for cell in row {
                match cell {
                    Cell::Trail(Rope::Tail | Rope::Both) => {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }
        count
    }

    fn leave_cell(&mut self, x: usize, y: usize, rope: Rope) {
        let cell = self.map[y][x];
        match rope {
            Rope::Head => match cell {
                Cell::None => {
                    self.map[y][x] = Cell::Trail(Rope::Head);
                }
                Cell::Trail(Rope::Head | Rope::Both) => {}
                Cell::Trail(Rope::Tail) => {
                    self.map[y][x] = Cell::Trail(Rope::Both);
                }
            },
            Rope::Tail => match cell {
                Cell::None => {
                    self.map[y][x] = Cell::Trail(Rope::Tail);
                }
                Cell::Trail(Rope::Tail | Rope::Both) => {}
                Cell::Trail(Rope::Head) => {
                    self.map[y][x] = Cell::Trail(Rope::Both);
                }
            },
            Rope::Both => panic!("Both cannot leave cell simultaneously"),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        self.maybe_expand(dir);
        match dir {
            Direction::Up(val) => {
                let y = self.head.y - val;
                for i in (y..=self.head.y).rev() {
                    self.leave_cell(self.head.x, i, Rope::Head);
                    self.move_tail(self.head.x, i);
                }
                self.move_tail(self.head.x, y);
                self.head.y = y;
            }
            Direction::Down(val) => {
                let y = self.head.y + val;
                for i in self.head.y..y {
                    self.leave_cell(self.head.x, i, Rope::Head);
                    self.move_tail(self.head.x, i);
                }
                self.move_tail(self.head.x, y);
                self.head.y = y;
            }
            Direction::Left(val) => {
                let x = self.head.x - val;
                for i in (x..=self.head.x).rev() {
                    self.leave_cell(i, self.head.y, Rope::Head);
                    self.move_tail(i, self.head.y);
                }
                self.move_tail(x, self.head.y);
                self.head.x = x;
            }
            Direction::Right(val) => {
                let x = self.head.x + val;
                for i in self.head.x..x {
                    self.leave_cell(i, self.head.y, Rope::Head);
                    self.move_tail(i, self.head.y);
                }
                self.move_tail(x, self.head.y);
                self.head.x = x;
            }
        }
    }

    fn move_tail(&mut self, head_x: usize, head_y: usize) {
        let Position {
            x: tail_x,
            y: tail_y,
        } = self.tail;
        self.leave_cell(tail_x, tail_y, Rope::Tail);
        let xdiff = head_x as isize - tail_x as isize;
        let ydiff = head_y as isize - tail_y as isize;

        let (mut itail_x, mut itail_y) = (tail_x as isize, tail_y as isize);

        match (xdiff.abs(), ydiff.abs()) {
            (0, 0) | (1, 0) | (0, 1) | (1, 1) => {}
            (2, 0) => itail_x += xdiff / 2,
            (0, 2) => itail_y += ydiff / 2,
            (1, 2) => {
                itail_x += xdiff;
                itail_y += ydiff / 2;
            }
            (2, 1) => {
                itail_x += xdiff / 2;
                itail_y += ydiff;
            }
            other => panic!("Unexpected movement: {other:?}"),
        };
        self.tail = Position {
            x: itail_x as usize,
            y: itail_y as usize,
        };
    }

    // Handles expanding the map in the direction of movement if required
    fn maybe_expand(&mut self, dir: Direction) {
        match dir {
            Direction::Up(val) => {
                let diff = self.head.y as isize - val as isize;
                if diff < 0 {
                    for _ in 0..(diff.abs()) {
                        let mut row = self.map[0].clone();
                        for i in 0..row.len() {
                            row[i] = Cell::None;
                        }
                        self.map.push_front(row);
                    }
                    self.head.y = self.head.y + diff.abs() as usize;
                    self.tail.y = self.tail.y + diff.abs() as usize;
                }
            }
            Direction::Down(val) => {
                let diff = (self.head.y + val) as isize - (self.map.len() - 1) as isize;
                if diff > 0 {
                    for _ in 0..diff {
                        let mut row = self.map[0].clone();
                        for i in 0..row.len() {
                            row[i] = Cell::None;
                        }
                        self.map.push_back(row);
                    }
                }
            }
            Direction::Left(val) => {
                let diff = self.head.x as isize - val as isize;
                if diff < 0 {
                    for _ in 0..(diff.abs()) {
                        for i in 0..self.map.len() {
                            self.map[i].push_front(Cell::None);
                        }
                    }
                    self.head.x = self.head.x + diff.abs() as usize;
                    self.tail.x = self.tail.x + diff.abs() as usize;
                }
            }
            Direction::Right(val) => {
                let diff = (self.head.x + val) as isize - (self.map[0].len() - 1) as isize;
                if diff > 0 {
                    for _ in 0..diff {
                        for i in 0..self.map.len() {
                            self.map[i].push_back(Cell::None);
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for GrowingMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                let pos = Position { x, y };
                if pos == self.head && pos == self.tail {
                    write!(f, "[{}]", Rope::Both)?;
                } else if pos == self.head {
                    write!(f, "[{}]", Rope::Head)?;
                } else if pos == self.tail {
                    write!(f, "[{}]", Rope::Tail)?;
                } else {
                    write!(f, "[{col}]")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

#[derive(Debug, Clone, Copy)]
enum Rope {
    Head,
    Tail,
    Both,
}

impl FromStr for Rope {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            "H" => Rope::Head,
            "T" => Rope::Tail,
            "B" => Rope::Both,
            _ => return Err(anyhow!("Invalid rope value {s}")),
        };
        Ok(val)
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Rope::Head => "H",
            Rope::Tail => "T",
            Rope::Both => "B",
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    None,
    Trail(Rope),
}

impl FromStr for Cell {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let val = match s {
            " " => Cell::None,
            "." => Cell::Trail(Rope::Head),
            "," => Cell::Trail(Rope::Tail),
            ";" => Cell::Trail(Rope::Both),
            _ => return Err(anyhow!("Invalid cell value {s}")),
        };
        Ok(val)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Cell::None => " ",
            Cell::Trail(rope) => match rope {
                Rope::Head => ".",
                Rope::Tail => ",",
                Rope::Both => ";",
            },
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for Direction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if let [d, val] = &s.split(" ").collect::<Vec<_>>()[..] {
            let num = val
                .parse::<usize>()
                .map_err(|_| anyhow!("invalid value {val}"))?;
            let dir = match *d {
                "U" => Direction::Up(num),
                "D" => Direction::Down(num),
                "L" => Direction::Left(num),
                "R" => Direction::Right(num),
                _ => return Err(anyhow!("Invalid letter {d}")),
            };
            return Ok(dir);
        }
        Err(anyhow!("Invalid direction: {s}"))
    }
}

fn read_input(path: &Path) -> Result<Vec<Direction>> {
    let mut input = String::new();
    let mut f = StdFile::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|x| Direction::from_str(x).unwrap())
        .collect())
}
