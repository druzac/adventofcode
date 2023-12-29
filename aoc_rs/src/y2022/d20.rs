use crate::{AOCError, ProblemPart};

use std::io;

#[derive(Debug, PartialEq, Eq)]
struct MixListElement {
    value: i64,
    mixed: bool,
}

impl MixListElement {
    fn new(val: i64) -> MixListElement {
        MixListElement {
            value: val,
            mixed: false,
        }
    }

    fn new_mixed(val: i64) -> MixListElement {
        MixListElement {
            value: val,
            mixed: true,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct UpperNode {
    left_size: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Leaf(Vec<MixListElement>),
    Upper(UpperNode),
}

#[derive(Debug)]
struct MixList {
    nodes: Vec<Node>,
}

impl MixList {

    fn new(v: &[i64], chunk_size: usize) -> MixList {
        let num_leaves = v.len().div_ceil(chunk_size);
        if v.is_empty() || num_leaves == 1 {
            return MixList {
                nodes: vec![Node::Upper(UpperNode { left_size: v.len() }),
                            Node::Leaf(v.iter().map(|x| MixListElement::new(*x)).collect())]
            }
        }
        let num_leaves = v.len().div_ceil(chunk_size);
        if num_leaves.is_power_of_two() {
            let mut leaves = Vec::with_capacity(num_leaves);
            let mut current_leaf = Vec::with_capacity(chunk_size);
            for el in v.iter() {
                if current_leaf.len() >= chunk_size {
                    leaves.push(current_leaf);
                    current_leaf = Vec::new();
                }
                current_leaf.push(MixListElement::new(*el));
            }
            leaves.push(current_leaf);
            let mut current_layer = Vec::with_capacity(leaves.len() / 2);
            for i in (0..leaves.len()).step_by(2) {
                let left_size = leaves[i].len();
                let right_size = leaves.get(i + 1).map(|v| v.len()).unwrap_or(0);
                current_layer.push((UpperNode { left_size: left_size }, right_size));
            }
            let mut upper_layers = Vec::new();
            let mut last_layer = current_layer;
            while last_layer.len() > 1 {
                current_layer = Vec::with_capacity(last_layer.len() / 2);
                for i in (0..last_layer.len()).step_by(2) {
                    let left_size = last_layer[i].0.left_size + last_layer[i].1;
                    let right_size = last_layer.get(i + 1).map(|(mid, rsize)| mid.left_size + rsize).unwrap_or(0);
                    current_layer.push((UpperNode { left_size: left_size }, right_size));
                }
                upper_layers.push(last_layer);
                last_layer = current_layer;
            }
            let mut nodes = Vec::with_capacity(num_leaves * 2);
            match last_layer.pop() {
                Some((node, _)) => nodes.push(Node::Upper(node)),
                _ => panic!(),
            }
            for layer in upper_layers.into_iter().rev() {
                for (node, _) in layer.into_iter() {
                    nodes.push(Node::Upper(node));
                }
            }
            for leaf in leaves.into_iter() {
                nodes.push(Node::Leaf(leaf));
            }
            MixList { nodes: nodes }
        } else {
            let greatest_le_pow_2 = num_leaves.next_power_of_two() >> 1;
            let parents_in_second_last_layer = num_leaves - greatest_le_pow_2;
            let leaves_in_second_last_layer = greatest_le_pow_2 - parents_in_second_last_layer;
            let leaves_in_last_layer = num_leaves - leaves_in_second_last_layer;
            let mut bottom_leaves = Vec::with_capacity(leaves_in_last_layer);
            let mut current_leaf = Vec::with_capacity(chunk_size);
            for el in &v[0..leaves_in_last_layer * chunk_size] {
                if current_leaf.len() >= chunk_size {
                    bottom_leaves.push(current_leaf);
                    current_leaf = Vec::new();
                }
                current_leaf.push(MixListElement::new(*el));
            }
            bottom_leaves.push(current_leaf);
            let mut current_layer = Vec::with_capacity(bottom_leaves.len() / 2 + leaves_in_second_last_layer);
            for i in (0..bottom_leaves.len()).step_by(2) {
                let left_size = bottom_leaves[i].len();
                let right_size = bottom_leaves.get(i + 1).map(|v| v.len()).unwrap_or(0);
                current_layer.push((Node::Upper(UpperNode { left_size: left_size }), right_size));
            }
            // add the remaining leaves to the current layer
            current_leaf = Vec::with_capacity(chunk_size);
            for el in &v[leaves_in_last_layer * chunk_size..] {
                if current_leaf.len() >= chunk_size {
                    current_layer.push((Node::Leaf(current_leaf), 0));
                    current_leaf = Vec::new();
                }
                current_leaf.push(MixListElement::new(*el));
            }
            current_layer.push((Node::Leaf(current_leaf), 0));
            let mut last_layer = current_layer;
            let mut upper_layers = Vec::new();
            while last_layer.len() > 1 {
                current_layer = Vec::with_capacity(last_layer.len() / 2);
                for i in (0..last_layer.len()).step_by(2) {
                    let left_size = match &last_layer[i] {
                        (Node::Upper(parent), right_size) => parent.left_size + right_size,
                        (Node::Leaf(l), right_size) => l.len() + right_size,
                    };
                    let right_size = match last_layer.get(i + 1) {
                        None => 0,
                        Some((Node::Upper(parent), right_size)) => parent.left_size + right_size,
                        Some((Node::Leaf(l), right_size)) => l.len() + right_size,
                    };
                    current_layer.push((Node::Upper(UpperNode { left_size: left_size }), right_size));
                }
                upper_layers.push(last_layer);
                last_layer = current_layer;
            }
            let mut nodes = Vec::with_capacity(num_leaves * 2);
            match last_layer.pop() {
                Some((node, _)) => nodes.push(node),
                _ => panic!(),
            }
            for layer in upper_layers.into_iter().rev() {
                for (node, _) in layer.into_iter() {
                    nodes.push(node);
                }
            }
            for leaf in bottom_leaves.into_iter() {
                nodes.push(Node::Leaf(leaf));
            }
            MixList { nodes: nodes }
        }
    }

    fn get(&self, idx: usize) -> Option<&MixListElement> {
        let mut element_idx = idx;
        let mut node_idx = 0;
        let mut current_node = Some(&self.nodes[node_idx]);
        while let Some(Node::Upper(next_node)) = current_node {
            match element_idx.checked_sub(next_node.left_size) {
                Some(delta) => {
                    element_idx = delta;
                    node_idx = node_idx * 2 + 2;
                    current_node = self.nodes.get(node_idx);
                },
                None => {
                    node_idx = node_idx * 2 + 1;
                    current_node = self.nodes.get(node_idx);
                }
            }
        }
        match current_node {
            Some(Node::Leaf(v)) => v.get(element_idx),
            Some(Node::Upper(_)) => panic!("bug!"),
            None => None,
        }
    }

    fn remove_unmixed(&mut self, idx: usize) -> Option<MixListElement> {
        self.remove_unmixed_helper(idx, 0)
    }

    fn remove_unmixed_helper(&mut self, element_idx: usize, node_idx: usize) -> Option<MixListElement> {
        let mut went_left = false;
        let result = match self.nodes.get_mut(node_idx) {
            Some(Node::Upper(next_node)) => {
                match element_idx.checked_sub(next_node.left_size) {
                    None => {
                        went_left = true;
                        self.remove_unmixed_helper(element_idx, node_idx * 2 + 1)
                    },
                    Some(delta) => self.remove_unmixed_helper(delta, node_idx * 2 + 2),
                }
            },
            Some(Node::Leaf(v)) => {
                if v.get(element_idx).map_or(false, |el| !el.mixed) {
                    Some(v.remove(element_idx))
                } else {
                    None
                }
            },
            None => None,
        };
        if went_left && result.is_some() {
            match self.nodes.get_mut(node_idx) {
                Some(Node::Upper(node)) => node.left_size -= 1,
                _ => panic!("bug when setting went_left!"),
            }
        }
        result
    }

    // panics if idx > len
    fn insert(&mut self, idx: usize, el: MixListElement) {
        self.insert_helper(idx, 0, el)
    }

    fn insert_helper(&mut self, element_idx: usize, node_idx: usize, el: MixListElement) {
        let mut went_left = false;
        match self.nodes.get_mut(node_idx) {
            Some(Node::Upper(next_node)) => {
                match element_idx.checked_sub(next_node.left_size) {
                    None | Some(0) => {
                        went_left = true;
                        self.insert_helper(element_idx, node_idx * 2 + 1, el)
                    },
                    Some(delta) => {
                        self.insert_helper(delta, node_idx * 2 + 2, el)
                    },
                }
            },
            Some(Node::Leaf(v)) => {
                v.insert(element_idx, el);
            },
            None => (),
        }
        if went_left {
            match self.nodes.get_mut(node_idx) {
                Some(Node::Upper(node)) => node.left_size += 1,
                _ => panic!("bug when setting went_left!"),
            }
        }
    }

    fn print(&self) {
        for (i, node) in self.nodes.iter().enumerate() {
            println!("{}: {:?}", i, node);
        }
    }

    fn iter(&self) -> MixListIterator {
        MixListIterator {
            mixlist: self,
            node_idx_stack: vec![0],
            current_leaf: None,
        }
    }

    // fn find(&self, val: i64) -> Option<usize> {
    // }

    // // hmm... not sure how to compute the idx.
    // fn find_helper(&self, val: i64, node_idx: usize) -> Option<usize> {
    //     match self.nodes.get(node_idx) => {
    //         Some(Node::Upper(next_node)) => self.find_helper(val, node_idx * 2 + 1).or_else(|| self.find_helper(val, node_idx * 2 + 2)),
    //         Some(Node::Leaf(v)) =>
    //             // match self.find_helper(val, node_idx * 2 + 1) {
    //             // }
    //         }
    //     }
    // }
}

struct MixListIterator<'a> {
    mixlist: &'a MixList,
    node_idx_stack: Vec<usize>,
    current_leaf: Option<(&'a Vec<MixListElement>, usize)>,
}

impl<'a> Iterator for MixListIterator<'a> {
    type Item = &'a MixListElement;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((v, v_idx)) = self.current_leaf.take() {
            if let Some(el) = v.get(v_idx) {
                self.current_leaf = Some((v, v_idx + 1));
                return Some(el);
            }
        }
        while let Some(node_idx) = self.node_idx_stack.pop() {
            match self.mixlist.nodes.get(node_idx) {
                Some(Node::Upper(_)) => {
                    self.node_idx_stack.push(node_idx * 2 + 2);
                    self.node_idx_stack.push(node_idx * 2 + 1);
                },
                Some(Node::Leaf(v)) => {
                    self.current_leaf = Some((v, 0));
                    return self.next()
                },
                None => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_single() {
        let mixlist = MixList::new(&[1], 2);
        assert_eq!(mixlist.nodes.len(), 2);
        assert_eq!(mixlist.nodes[0], Node::Upper(UpperNode { left_size: 1 }));
        assert_eq!(mixlist.nodes[1], Node::Leaf(vec![MixListElement::new(1)]));
    }

    #[test]
    fn test_get_filled_leaf_layer() {
        let mixlist = MixList::new(&[1, 2, 3, 4, 5, 6, 7, 8], 2);
        for idx in 0..8 {
            assert_eq!(mixlist.get(idx), Some(&MixListElement::new(idx as i64 + 1)));
        }
        assert_eq!(mixlist.get(8), None);
    }

    #[test]
    fn test_get_unfilled_leaf_layer() {
        let mixlist = MixList::new(&[1, 2, 3, 4, 5, 6, 7], 2);
        for idx in 0..7 {
            assert_eq!(mixlist.get(idx), Some(&MixListElement::new(idx as i64 + 1)));
        }
        assert_eq!(mixlist.get(7), None);
    }

    #[test]
    fn test_remove_unmixed() {
        for idx in 0..7 {
            let mut mixlist = MixList::new(&(0..7).collect::<Vec<i64>>(), 2);
            assert_eq!(mixlist.remove_unmixed(idx), Some(MixListElement::new(idx as i64)));
            for get_idx in 0..6 {
                let expected = if get_idx >= idx {
                    (get_idx + 1) as i64
                } else {
                    get_idx as i64
                };
                assert_eq!(mixlist.get(get_idx), Some(&MixListElement::new(expected)));
            }
        }
    }

    #[test]
    fn test_iterator() {
        let source_vec = (0..16).collect::<Vec<i64>>();
        let mixlist = MixList::new(&source_vec, 2);
        assert_eq!(mixlist.iter().map(|el| el.value).collect::<Vec<i64>>(),
                   source_vec);
    }

    #[test]
    fn test_imperfect_trees() {
        for total_elements in 10..20 {
            let range = 0..total_elements;
            let mixlist = MixList::new(&range.clone().collect::<Vec<i64>>(), 2);
            mixlist.print();
            for i in 0..total_elements {
                assert_eq!(mixlist.get(i as usize), Some(&MixListElement::new(i as i64)));
            }
            assert_eq!(mixlist.iter().map(|el| el.value).collect::<Vec<i64>>(),
                       range.collect::<Vec<i64>>());
        }
    }
}

fn parse_problem<B: io::BufRead>(br: B) -> Result<Vec<i64>, AOCError> {
    let mut result = Vec::new();
    for res_line in br.lines() {
        result.push(res_line?.parse::<i64>()?);
    }
    Ok(result)
}

fn problem1(raw_vec: Vec<i64>) -> i64 {
    let chunk_size = if raw_vec.len() <= 50 {
        2
    } else {
        100
    };
    let mut mixlist = MixList::new(&raw_vec, chunk_size);
    let mut idx = 0;
    let total_size = raw_vec.len();
    let size_but_one = total_size as i64 - 1;
    while idx < total_size {
        let el = match mixlist.remove_unmixed(idx) {
            Some(el) => el,
            None => { idx += 1; continue; },
        };
        let new_el = MixListElement::new_mixed(el.value);
        let mut new_idx = idx as i64 + el.value;
        while new_idx <= 0 {
            new_idx += size_but_one as i64;
        }
        while new_idx > size_but_one as i64 {
            new_idx -= size_but_one as i64;
        }
        assert!(new_idx >= 1 && new_idx <= size_but_one as i64);
        mixlist.insert(new_idx as usize, new_el);
    }
    let idx = mixlist.iter().enumerate().find_map(|(idx, el)| if el.value == 0 { Some(idx) } else { None }).unwrap();
    let mut sum = 0;
    for delta in [1000, 2000, 3000] {
        sum += mixlist.get((idx + delta) % total_size).unwrap().value;
    }
    sum
}

// problem2
// the numbers aren't all unique, so we need to keep track somehow.

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let raw_vec = parse_problem(br)?;
    let result = match part {
        ProblemPart::P1 => problem1(raw_vec),
        ProblemPart::P2 => 0,
    };
    println!("result: {}", result);
    Ok(())
}
