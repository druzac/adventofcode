use crate::{AOCError, ProblemPart};

use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct MixListElement {
    value: i64,
    original_idx: usize,
}

impl MixListElement {
    fn new(val: i64, idx: usize) -> MixListElement {
        MixListElement {
            value: val,
            original_idx: idx,
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
    coord_to_node_idx: HashMap<MixListElement, usize>,
}

fn make_parents(children: &[(Node, usize)]) -> Vec<(Node, usize)> {
    let mut parents = Vec::with_capacity(children.len() / 2);
    for i in (0..children.len()).step_by(2) {
        let left_size = match &children[i] {
            (Node::Upper(parent), right_size) => parent.left_size + right_size,
            (Node::Leaf(l), right_size) => l.len() + right_size,
        };
        let right_size = match children.get(i + 1) {
            None => 0,
            Some((Node::Upper(parent), right_size)) => parent.left_size + right_size,
            Some((Node::Leaf(l), right_size)) => l.len() + right_size,
        };
        parents.push((
            Node::Upper(UpperNode {
                left_size: left_size,
            }),
            right_size,
        ));
    }
    parents
}

fn reduce(values: &[i64], val: i64) -> i64 {
    let size_but_one = values.len() as i64 - 1;
    let result = val % size_but_one;
    if result <= 0 {
        result + size_but_one
    } else {
        result
    }
}

fn make_leaves(values: &[i64], chunk_size: usize, idx_offset: usize) -> Vec<(Node, usize)> {
    if values.is_empty() {
        return Vec::new();
    };
    let mut leaves = Vec::with_capacity(values.len().div_ceil(chunk_size));
    let mut current_leaf = Vec::with_capacity(chunk_size);
    for (idx, el) in values.iter().enumerate() {
        if current_leaf.len() >= chunk_size {
            leaves.push((Node::Leaf(current_leaf), 0));
            current_leaf = Vec::new();
        }
        current_leaf.push(MixListElement::new(*el, idx + idx_offset));
    }
    leaves.push((Node::Leaf(current_leaf), 0));
    leaves
}

impl MixList {
    fn new(v: &[i64], chunk_size: usize) -> MixList {
        if v.is_empty() {
            return MixList {
                nodes: vec![Node::Upper(UpperNode { left_size: 0 }),
                            Node::Leaf(Vec::new())],
                coord_to_node_idx: HashMap::new(),
            }
        }
        let num_leaves = v.len().div_ceil(chunk_size);
        let (first_slice, second_slice): ((&[i64], usize), (&[i64], usize)) =
            if num_leaves.is_power_of_two() {
                ((v, 0), (&[], 0))
            } else {
                let greatest_le_pow_2 = num_leaves.next_power_of_two() >> 1;
                let parents_in_second_last_layer = num_leaves - greatest_le_pow_2;
                let leaves_in_second_last_layer = greatest_le_pow_2 - parents_in_second_last_layer;
                let leaves_in_last_layer = num_leaves - leaves_in_second_last_layer;
                let elements_in_bottom_leaves = leaves_in_last_layer * chunk_size;
                (
                    (&v[..elements_in_bottom_leaves], 0),
                    (&v[elements_in_bottom_leaves..], elements_in_bottom_leaves),
                )
            };
        let mut layers = Vec::new();
        let mut previous_layer = make_leaves(first_slice.0, chunk_size, first_slice.1);
        let mut current_layer = make_parents(&previous_layer);
        current_layer.append(&mut make_leaves(second_slice.0, chunk_size, second_slice.1));
        layers.push(previous_layer);
        previous_layer = current_layer;
        while previous_layer.len() > 1 {
            let current_layer = make_parents(&previous_layer);
            layers.push(previous_layer);
            previous_layer = current_layer;
        }
        let mut nodes = Vec::with_capacity(num_leaves * 2);
        match previous_layer.pop() {
            Some((node, _)) => nodes.push(node),
            _ => panic!(),
        }
        for layer in layers.into_iter().rev() {
            for (node, _) in layer.into_iter() {
                nodes.push(node);
            }
        }
        let mut map = HashMap::with_capacity(v.len());
        for (node_idx, node) in nodes.iter().enumerate() {
            if let Node::Leaf(chunk) = node {
                for mix_list_element in chunk.iter() {
                    map.insert(*mix_list_element, node_idx);
                }
            }
        }
        MixList {
            nodes: nodes,
            coord_to_node_idx: map,
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
                }
                None => {
                    node_idx = node_idx * 2 + 1;
                    current_node = self.nodes.get(node_idx);
                }
            }
        }
        match current_node {
            Some(Node::Leaf(v)) => v.get(element_idx),
            Some(Node::Upper(_)) => panic!(),
            None => None,
        }
    }

    // panics if idx > len
    fn insert(&mut self, idx: usize, el: MixListElement) {
        self.insert_helper(idx, 0, el)
    }

    fn insert_helper(&mut self, element_idx: usize, node_idx: usize, el: MixListElement) {
        let mut went_left = false;
        match self.nodes.get_mut(node_idx) {
            Some(Node::Upper(next_node)) => match element_idx.checked_sub(next_node.left_size) {
                None | Some(0) => {
                    went_left = true;
                    self.insert_helper(element_idx, node_idx * 2 + 1, el)
                }
                Some(delta) => self.insert_helper(delta, node_idx * 2 + 2, el),
            },
            Some(Node::Leaf(v)) => {
                self.coord_to_node_idx.insert(el, node_idx);
                v.insert(element_idx, el);
            }
            None => (),
        }
        if went_left {
            match self.nodes.get_mut(node_idx) {
                Some(Node::Upper(node)) => node.left_size += 1,
                _ => panic!(),
            }
        }
    }

    fn iter(&self) -> MixListIterator {
        MixListIterator {
            mixlist: self,
            node_idx_stack: vec![0],
            current_leaf: None,
        }
    }

    // return the index the element was found at, or None if it wasn't found.
    fn remove_coord(&mut self, remove_el: MixListElement) -> Option<(usize, MixListElement)> {
        let mut node_index = if let Some(node_index) = self.coord_to_node_idx.get(&remove_el) {
            *node_index
        } else {
            return None;
        };
        let (mut element_index, mix_list_el) = match self.nodes.get_mut(node_index) {
            Some(Node::Leaf(vals)) => {
                let maybe_idx = vals.iter().enumerate().find_map(|(idx, el)| {
                    if el == &remove_el {
                        Some(idx)
                    } else {
                        None
                    }
                });
                let idx = match maybe_idx {
                    Some(i) => i,
                    None => return None,
                };
                (idx, vals.remove(idx))
            }
            _ => panic!(),
        };
        while node_index > 0 {
            let left_child = if node_index % 2 != 0 { true } else { false };
            node_index = (node_index - 1) / 2;
            match self.nodes.get_mut(node_index) {
                Some(Node::Upper(parent)) => {
                    if left_child {
                        parent.left_size -= 1;
                    } else {
                        element_index += parent.left_size;
                    }
                }
                _ => panic!(),
            };
        }
        Some((element_index, mix_list_el))
    }

    fn mix(&mut self, original_elements: &[i64]) {
        let size_but_one = original_elements.len() as i64 - 1;
        for (original_idx, val) in original_elements.iter().enumerate() {
            let (current_idx, ml_el) = self
                .remove_coord(MixListElement::new(*val, original_idx))
                .unwrap();
            let new_idx = reduce(original_elements, current_idx as i64 + val);
            assert!(new_idx >= 1 && new_idx <= size_but_one);
            self.insert(new_idx as usize, ml_el);
        }
    }
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
                }
                Some(Node::Leaf(v)) => {
                    self.current_leaf = Some((v, 0));
                    return self.next();
                }
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
        assert_eq!(
            mixlist.nodes[1],
            Node::Leaf(vec![MixListElement::new(1, 0)])
        );
    }

    #[test]
    fn test_get_filled_leaf_layer() {
        let mixlist = MixList::new(&[1, 2, 3, 4, 5, 6, 7, 8], 2);
        for idx in 0..8 {
            let val = (idx + 1) as i64;
            assert_eq!(mixlist.get(idx), Some(&MixListElement::new(val, idx)));
        }
        assert_eq!(mixlist.get(8), None);
    }

    #[test]
    fn test_get_unfilled_leaf_layer() {
        let mixlist = MixList::new(&[1, 2, 3, 4, 5, 6, 7], 2);
        for idx in 0..6 {
            assert_eq!(
                mixlist.get(idx),
                Some(&MixListElement::new((idx + 1) as i64, idx))
            );
        }
        assert_eq!(mixlist.get(7), None);
    }

    #[test]
    fn test_iterator() {
        let source_vec = (0..16).collect::<Vec<i64>>();
        let mixlist = MixList::new(&source_vec, 2);
        assert_eq!(
            mixlist.iter().map(|el| el.value).collect::<Vec<i64>>(),
            source_vec
        );
    }

    #[test]
    fn test_remove_coord() {
        let source_vec = (0..7).collect::<Vec<i64>>();
        let mut mixlist = MixList::new(&source_vec, 2);
        let result = mixlist.remove_coord(MixListElement::new(1, 1));
        assert_eq!(result, Some((1, MixListElement::new(1, 1))));
        assert_eq!(
            mixlist.remove_coord(MixListElement::new(2, 2)),
            Some((1, MixListElement::new(2, 2)))
        );
    }
}

fn parse_problem<B: io::BufRead>(br: B) -> Result<Vec<i64>, AOCError> {
    let mut result = Vec::new();
    for res_line in br.lines() {
        result.push(res_line?.parse::<i64>()?);
    }
    Ok(result)
}

fn get_chunk_size(raw: &[i64]) -> usize {
    if raw.len() <= 50 {
        2
    } else {
        125
    }
}

fn get_grove_coordinates(raw_vec: &[i64], mixlist: &MixList) -> i64 {
    let idx = mixlist.iter().position(|el| el.value == 0).unwrap();
    let mut sum = 0;
    let total_size = raw_vec.len();
    for delta in [1000, 2000, 3000] {
        sum += mixlist.get((idx + delta) % total_size).unwrap().value;
    }
    sum
}

fn problem1(raw_vec: Vec<i64>) -> i64 {
    let chunk_size = get_chunk_size(&raw_vec);
    let mut mixlist = MixList::new(&raw_vec, chunk_size);
    mixlist.mix(&raw_vec);
    get_grove_coordinates(&raw_vec, &mixlist)
}

fn problem2(mut raw_vec: Vec<i64>) -> i64 {
    let chunk_size = get_chunk_size(&raw_vec);
    let decryption_key = 811589153;
    for val in raw_vec.iter_mut() {
        *val = *val * decryption_key;
    }
    let mut mixlist = MixList::new(&raw_vec, chunk_size);
    for _ in 0..10 {
        mixlist.mix(&raw_vec);
    }
    get_grove_coordinates(&raw_vec, &mixlist)
}

pub fn solve<B: io::BufRead>(part: ProblemPart, br: B) -> Result<(), AOCError> {
    let raw_vec = parse_problem(br)?;
    let result = match part {
        ProblemPart::P1 => problem1(raw_vec),
        ProblemPart::P2 => problem2(raw_vec),
    };
    println!("result: {}", result);
    Ok(())
}
