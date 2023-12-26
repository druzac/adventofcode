use crate::{aocerror, AOCError, ProblemPart};

use std::io;

fn is_stack_line(s: &str) -> bool {
    s.trim_start().split(' ').next().map_or(false, |c| c == "1")
}

fn find_stack_name_line(lines: &[String]) -> Option<usize> {
    lines
        .iter()
        .enumerate()
        .find_map(|(idx, l)| if is_stack_line(l) { Some(idx) } else { None })
}

fn transpose(lines: &[String]) -> Vec<String> {
    let mut char_iters: Vec<_> = lines.iter().map(|l| l.chars()).collect();
    let mut new_lines: Vec<String> = Vec::new();
    loop {
        let mut new_s = String::new();
        for it in char_iters.iter_mut() {
            it.next().map(|c| new_s.push(c));
        }
        if new_s.is_empty() {
            break;
        }
        new_lines.push(new_s);
    }
    new_lines
}

fn string_trim_end(s: &mut String) {
    let new_len = s.trim_end().len();
    s.truncate(new_len)
}

fn get_stack_contents(stack_content_lines: &mut [String]) -> Vec<String> {
    stack_content_lines.reverse();
    transpose(stack_content_lines)
        .into_iter()
        .filter(|l| !l.starts_with(&[' ', '[', ']']))
        .map(|mut s| {
            string_trim_end(&mut s);
            s
        })
        .collect()
}

fn parse_stacks(lines: &mut [String]) -> Vec<String> {
    let line_length = lines.len();
    let stack_contents = get_stack_contents(&mut lines[0..line_length - 1]);
    stack_contents
}

#[derive(Debug)]
struct Command {
    count: u64,
    source: u64,
    destination: u64,
}

fn parse_command(line: &str) -> Command {
    let els: Vec<_> = line.split(' ').collect();
    assert!(els.len() == 6);
    assert!(els[0] == "move");
    assert!(els[2] == "from");
    assert!(els[4] == "to");
    Command {
        count: els[1].parse::<u64>().unwrap(),
        source: els[3].parse::<u64>().unwrap(),
        destination: els[5].parse::<u64>().unwrap(),
    }
}

fn parse_commands(lines: &[String]) -> Vec<Command> {
    lines.iter().map(|l| parse_command(l)).collect()
}

fn apply_command(stacks: &mut [String], command: &Command) {
    let src_idx = (command.source - 1) as usize;
    let dst_idx = (command.destination - 1) as usize;
    for _ in 0..command.count {
        let val = stacks[src_idx].pop().unwrap();
        stacks[dst_idx].push(val)
    }
}

fn apply_command2(stacks: &mut [String], command: &Command) {
    let src_idx = (command.source - 1) as usize;
    let dst_idx = (command.destination - 1) as usize;
    let block: String = stacks[src_idx]
        .chars()
        .rev()
        .take(command.count as usize)
        .collect();
    let new_src_length = stacks[src_idx].len() - (command.count as usize);
    stacks[src_idx].truncate(new_src_length);
    for ch in block.chars().rev() {
        stacks[dst_idx].push(ch);
    }
}

fn execute(mut stacks: Vec<String>, commands: Vec<Command>, problem1: bool) -> String {
    for com in commands {
        if problem1 {
            apply_command(&mut stacks, &com);
        } else {
            apply_command2(&mut stacks, &com);
        }
    }
    let mut s = String::with_capacity(stacks.len());
    for mut stack in stacks {
        s.push(stack.pop().unwrap());
    }
    s
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let mut lines: Vec<String> = br.lines().map(|s| s.unwrap()).collect();
    let idx = find_stack_name_line(&lines).unwrap();
    let stacks = parse_stacks(&mut lines[0..idx + 1]);
    if lines[idx + 1] != "" {
        return Err(aocerror!("expected empty line, got: {}", lines[idx + 1]));
    }
    let commands = parse_commands(&lines[idx + 2..]);
    println!(
        "result: {}",
        execute(stacks, commands, part == ProblemPart::P1)
    );
    Ok(())
}
