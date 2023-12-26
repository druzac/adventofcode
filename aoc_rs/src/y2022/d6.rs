use crate::{aocerror, AOCError, ProblemPart};

use std::collections::HashMap;
use std::io;

struct MultiSet {
    inner: HashMap<u8, u64>,
}

impl MultiSet {
    fn new() -> MultiSet {
        MultiSet {
            inner: HashMap::new(),
        }
    }

    fn insert(&mut self, val: u8) {
        self.inner.entry(val).and_modify(|v| *v += 1).or_insert(1);
    }

    fn remove(&mut self, val: u8) {
        let mut new_count: Option<u64> = None;
        self.inner.entry(val).and_modify(|v| {
            *v -= 1;
            new_count = Some(*v)
        });
        if new_count == Some(0) {
            self.inner.remove(&val);
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

fn problem2(s: String, n: usize) -> usize {
    let signal: Vec<u8> = Vec::from(s);
    let mut set = MultiSet::new();
    for b in &signal[0..n] {
        set.insert(*b);
    }
    for i in n..signal.len() {
        if set.len() == n {
            return i;
        }
        set.remove(signal[i - n]);
        set.insert(signal[i]);
    }
    panic!("sequence of {} different characters never found!", n);
}

pub fn solve<B: io::BufRead>(part: ProblemPart, mut br: B) -> Result<(), AOCError> {
    let mut line = String::new();
    let read_bytes = br.read_line(&mut line)?;
    if read_bytes == 0 {
        return Err(aocerror!("expected a non-empty line"));
    }
    let result = match part {
        ProblemPart::P1 => problem2(line, 4),
        ProblemPart::P2 => problem2(line, 14),
    };
    println!("result: {}", result);
    Ok(())
}
