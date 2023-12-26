use crate::{AOCError, ProblemPart};

use std::collections::HashMap;
use std::io;

struct DirectoryEntry {
    parent_idx: Option<usize>,
    files: Vec<(String, u64)>,
    subdirs: HashMap<String, usize>,
}

impl DirectoryEntry {
    fn new(parent_idx: Option<usize>) -> DirectoryEntry {
        DirectoryEntry {
            parent_idx: parent_idx,
            files: Vec::new(),
            subdirs: HashMap::new(),
        }
    }
}

struct FileSystem {
    directories: Vec<DirectoryEntry>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            directories: vec![DirectoryEntry::new(None)],
        }
    }

    fn get_root(&self) -> usize {
        0 as usize
    }

    fn get_parent(&self, dir_idx: usize) -> Option<usize> {
        self.directories[dir_idx].parent_idx
    }

    fn get_subdir(&self, dir_idx: usize, name: &str) -> Option<usize> {
        self.directories[dir_idx].subdirs.get(name).copied()
    }

    fn get_files(&self, dir_idx: usize) -> &Vec<(String, u64)> {
        &self.directories[dir_idx].files
    }

    fn get_subdirs(&self, dir_idx: usize) -> &HashMap<String, usize> {
        &self.directories[dir_idx].subdirs
    }

    fn add_subdirectory(&mut self, dir_idx: usize, name: String) {
        if let Some(_) = self.directories[dir_idx].subdirs.get(&name) {
            return;
        }
        let idx_of_new_entry = self.directories.len();
        self.directories[dir_idx]
            .subdirs
            .insert(name, idx_of_new_entry);
        self.directories.push(DirectoryEntry::new(Some(dir_idx)))
    }

    fn add_file(&mut self, dir_idx: usize, file_name: String, file_size: u64) {
        self.directories[dir_idx].files.push((file_name, file_size))
    }

    fn apply_command(&mut self, curr_dir_idx: usize, command: Command) -> usize {
        match command {
            Command::ChDir(ChDirTarget::Root) => self.get_root(),
            Command::ChDir(ChDirTarget::Parent) => self.get_parent(curr_dir_idx).unwrap(),
            Command::ChDir(ChDirTarget::SubDir(s)) => self.get_subdir(curr_dir_idx, &s).unwrap(),
            Command::LsDir(entries) => {
                for entry in entries {
                    match entry {
                        DirListing::File(fsize, fname) => self.add_file(curr_dir_idx, fname, fsize),
                        DirListing::Directory(dirname) => {
                            self.add_subdirectory(curr_dir_idx, dirname)
                        }
                    }
                }
                curr_dir_idx
            }
        }
    }
}

#[derive(Debug)]
enum DirListing {
    File(u64, String),
    Directory(String),
}

#[derive(Debug)]
enum ChDirTarget {
    Root,
    Parent,
    SubDir(String),
}

#[derive(Debug)]
enum Command {
    ChDir(ChDirTarget),
    LsDir(Vec<DirListing>),
}

fn parse_dir_listing(s: &str) -> Option<DirListing> {
    if s.chars().next().unwrap_or('$') == '$' {
        return None;
    }
    match s.split(' ').collect::<Vec<_>>()[..] {
        ["dir", dir_name] => Some(DirListing::Directory(dir_name.to_string())),
        [fsize, fname] => Some(DirListing::File(
            fsize.parse::<u64>().unwrap(),
            fname.to_string(),
        )),
        _ => panic!("oops"),
    }
}

fn parse_command(lines: &[String]) -> Option<(Command, &[String])> {
    if lines.is_empty() {
        return None;
    }
    match lines[0].split(' ').collect::<Vec<_>>()[..] {
        ["$", "cd", "/"] => Some((Command::ChDir(ChDirTarget::Root), &lines[1..])),
        ["$", "cd", ".."] => Some((Command::ChDir(ChDirTarget::Parent), &lines[1..])),
        ["$", "cd", dir_name] => Some((
            Command::ChDir(ChDirTarget::SubDir(dir_name.to_string())),
            &lines[1..],
        )),
        ["$", "ls"] => {
            let mut v: Vec<DirListing> = Vec::new();
            let mut i = 1 as usize;
            while i < lines.len() {
                if let Some(dlisting) = parse_dir_listing(&lines[i]) {
                    v.push(dlisting);
                } else {
                    break;
                }
                i += 1;
            }
            Some((Command::LsDir(v), &lines[i..]))
        }
        _ => panic!("oops"),
    }
}

fn get_directory_sizes_inner(fs: &FileSystem, curr_dir_idx: usize, sizes: &mut Vec<u64>) -> u64 {
    let mut directory_size = 0;
    for (_, fsize) in fs.get_files(curr_dir_idx) {
        directory_size += fsize;
    }
    for (_, subdiridx) in fs.get_subdirs(curr_dir_idx) {
        directory_size += get_directory_sizes_inner(fs, *subdiridx, sizes);
    }
    sizes.push(directory_size);
    directory_size
}

fn get_directory_sizes(fs: &FileSystem) -> Vec<u64> {
    let mut v = Vec::new();
    get_directory_sizes_inner(fs, fs.get_root(), &mut v);
    v
}

fn problem1(fs: &FileSystem) -> u64 {
    let mut sum_at_most_n = 0;
    for dir_size in get_directory_sizes(fs) {
        if dir_size <= 100000 {
            sum_at_most_n += dir_size;
        }
    }
    sum_at_most_n
}

fn problem2(fs: &FileSystem) -> u64 {
    let total_space = 70000000;
    let required_space = 30000000;
    let mut dir_sizes = get_directory_sizes(fs);
    let used_space = dir_sizes.iter().max().unwrap();
    let free_space = total_space - used_space;
    assert!(free_space < required_space);
    let space_to_free = required_space - free_space;
    dir_sizes.sort();
    dir_sizes.into_iter().find(|x| x >= &space_to_free).unwrap()
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let lines: Vec<String> = br.lines().collect::<Result<Vec<String>, io::Error>>()?;
    let mut ptr = &lines[..];
    let mut fs = FileSystem::new();
    let mut curr_dd = fs.get_root();
    while let Some((command, rest)) = parse_command(ptr) {
        curr_dd = fs.apply_command(curr_dd, command);
        ptr = rest;
    }
    let result = match part {
        ProblemPart::P1 => problem1(&fs),
        ProblemPart::P2 => problem2(&fs),
    };
    println!("result: {}", result);
    Ok(())
}
