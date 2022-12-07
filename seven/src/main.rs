use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
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
    let mut traversal = Traversal::new();
    let mut index = 0;
    while index < input.len() {
        traversal.handle_line(input[index].clone());
        index += 1
    }
    traversal.resolve_entries();
    traversal.sum_subdirs();
    let mut size = 0;
    for value in traversal.tree.values() {
        if *value <= 100000 {
            size += *value;
        }
    }
    println!("Size of dirs: {size}");

    const TOTAL_SIZE: u64 = 70000000;
    const MIN_SIZE: u64 = 30000000;
    let root = *traversal.tree.get("").unwrap();
    let unused_space = TOTAL_SIZE - root;
    let needed_size = MIN_SIZE - unused_space;
    let min_needed = traversal
        .tree
        .values()
        .filter(|&&val| val >= needed_size)
        .min()
        .unwrap();
    println!("Min size needed: {min_needed}");
    Ok(())
}

#[derive(Debug, Clone)]
enum Entry {
    File(File),
    Dir(String),
}

#[derive(Debug)]
struct Traversal {
    path: Vec<String>,
    entries: Vec<Entry>,
    tree: HashMap<String, u64>,
}

impl Traversal {
    fn new() -> Self {
        Self {
            path: Vec::new(),
            entries: Vec::new(),
            tree: HashMap::new(),
        }
    }

    fn handle_line(&mut self, line_type: LineType) {
        match line_type {
            LineType::Cd(cd_type) => {
                return match cd_type {
                    CdType::Dir(dir) => self.push_dir(dir),
                    CdType::Out => self.pop_dir(),
                    CdType::Root => self.go_root(),
                }
            }
            LineType::Ls => {}
            LineType::Entry(entry_type) => self.add_entry(entry_type),
        };
    }

    fn current_dir(&self) -> String {
        self.path.last().as_deref().unwrap().to_string()
    }

    fn push_dir(&mut self, dir: String) {
        self.resolve_entries();
        self.path.push(dir);
    }

    fn pop_dir(&mut self) {
        self.resolve_entries();
        self.path.pop().unwrap();
    }

    fn go_root(&mut self) {
        self.resolve_entries();
        self.push_dir("".into());
    }

    fn resolve_entries(&mut self) {
        let mut sum = 0;
        for entry in &self.entries {
            match entry {
                Entry::File(file) => {
                    sum += file.size;
                }
                Entry::Dir(dir) => {
                    let mut p = self.path.clone();
                    p.push(dir.clone());
                    let key = p.join("/");
                    if !self.tree.contains_key(dir) {
                        self.tree.insert(key, 0);
                    }
                }
            }
        }
        if self.path.is_empty() {
            return;
        }
        let key = self.path.join("/");
        if !self.tree.contains_key(&key) {
            self.tree.insert(key, sum);
        } else {
            *self.tree.get_mut(&key).unwrap() += sum;
        }
        self.entries = Vec::new();
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn sum_subdirs(&mut self) {
        let mut dirs = self.tree.keys().map(|x| x.to_string()).collect::<Vec<_>>();
        dirs.sort_by(|x, y| y.len().cmp(&x.len()));
        for (i, child) in dirs.iter().enumerate() {
            for parent in &dirs[i..] {
                let ccount = child.chars().filter(|&c| c == '/').count();
                let pcount = parent.chars().filter(|&c| c == '/').count();
                if pcount == ccount - 1 && child.contains(parent) {
                    let cval = *self.tree.get(child).unwrap();
                    *self.tree.get_mut(parent).unwrap() += cval;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum CdType {
    Dir(String),
    Out,
    Root,
}

impl FromStr for CdType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let out = match s {
            "cd /" => CdType::Root,
            "cd .." => CdType::Out,
            other => {
                let sp = other.split(" ").collect::<Vec<_>>();
                if let [action, dir] = &sp[..] {
                    if action.trim() != "cd" {
                        return Err(anyhow!("Invalid cd action: {other}"));
                    } else {
                        CdType::Dir(dir.to_string())
                    }
                } else {
                    return Err(anyhow!("Invalid cd action: {other}"));
                }
            }
        };
        Ok(out)
    }
}

#[derive(Debug, Clone)]
struct File {
    size: u64,
    name: String,
}

#[derive(Debug, Clone)]
enum LineType {
    Cd(CdType),
    Ls,
    Entry(Entry),
}

impl FromStr for LineType {
    type Err = Error;
    fn from_str(line: &str) -> Result<Self> {
        if line.starts_with("$ cd") {
            Ok(LineType::Cd(CdType::from_str(&line[2..])?))
        } else if line.starts_with("dir") {
            Ok(LineType::Entry(Entry::Dir(line.replace("dir ", ""))))
        } else if line.starts_with("$ ls") {
            Ok(LineType::Ls)
        } else {
            if let [size, name] = &line.split(" ").collect::<Vec<_>>()[..] {
                Ok(LineType::Entry(Entry::File(File {
                    size: size.parse::<u64>().expect(&format!("Invalid u64: {size}")),
                    name: name.to_string(),
                })))
            } else {
                Err(anyhow!("invalid line type: {line}"))
            }
        }
    }
}

fn read_input(path: &Path) -> Result<Vec<LineType>> {
    let mut input = String::new();
    let mut f = StdFile::open(path)?;
    f.read_to_string(&mut input)?;
    Ok(input
        .lines()
        .map(|x| LineType::from_str(x).unwrap())
        .collect())
}
