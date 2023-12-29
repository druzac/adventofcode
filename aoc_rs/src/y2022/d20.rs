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
    // right size is just set for easier construction, it's not used after.
    Upper(UpperNode),
}

#[derive(Debug)]
struct MixList {
    nodes: Vec<Node>,
}

impl MixList {
    // is a power of 2:
    // number of leaves:
    // v.len().div_ceil(chunk_size)
    // num.count_ones() == 1

    // this isn't right.
    fn new(v: &[i64], chunk_size: usize) -> MixList {
        let num_leaves = v.len().div_ceil(chunk_size);
        if v.is_empty() || num_leaves == 1 {
            // this is an edge case, don't worry for now.
            return MixList {
                nodes: vec![Node::Upper(UpperNode { left_size: v.len() }),
                            Node::Leaf(v.iter().map(|x| MixListElement::new(*x)).collect())]
            }
        }
        // new stuff
        let num_leaves = v.len().div_ceil(chunk_size);
        let greatest_le_pow_2 = if num_leaves.is_power_of_two() { num_leaves } else { num_leaves.next_power_of_two() >> 1 };
        let parents_in_second_last_layer = num_leaves - greatest_le_pow_2;
        let leaves_in_second_last_layer = greatest_le_pow_2 - parents_in_second_last_layer;
        let leaves_in_last_layer = num_leaves - leaves_in_second_last_layer;
        println!("num_leaves: {}, parents in second last layer: {}, leaves in second last layer: {}, leaves in last layer: {}", num_leaves,
                 parents_in_second_last_layer, leaves_in_second_last_layer, leaves_in_last_layer);
        let mut last_layer = Vec::new();
        let mut v_iter = v.iter();
        while leaves.len() < leaves_in_last_layer && v_iter {
            let mut current_leaf = vec![];
            while current_leaf.size() < chunk_size {
                if let Some(next_v) = v_iter.next() {
                    current_leaf.push();
                } else {
                    // need to break out of top level while
                }
            }
            for el in v.iter() {
                if current_leaf.len() >= chunk_size {
                    leaves.push(current_leaf);
                    current_leaf = Vec::new();
                }
                current_leaf.push(MixListElement::new(*el));
        }

        }
        leaves.push(current_leaf);
        // end new stuff

        // old stuff
        // let mut current_leaf = Vec::with_capacity(chunk_size);
        // for el in v.iter() {
        //     if current_leaf.len() >= chunk_size {
        //         leaves.push(current_leaf);
        //         current_leaf = Vec::new();
        //     }
        //     current_leaf.push(MixListElement::new(*el));
        // }
        // leaves.push(current_leaf);
        // now do the other layers
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
        // at this point, we have:
        // leaf_nodes
        // upper_layers
        // last_layer should be the root.
        let mut nodes = Vec::with_capacity(leaves.len() * 2);
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
        // println!("top level insert: {}", idx);
        self.insert_helper(idx, 0, el)
    }

    fn insert_helper(&mut self, element_idx: usize, node_idx: usize, el: MixListElement) {
        let mut went_left = false;
        match self.nodes.get_mut(node_idx) {
            Some(Node::Upper(next_node)) => {
                // don't use checked_sub here because we want to append in the happy case.
                match element_idx.checked_sub(next_node.left_size) {
                    // special case - append to the end on the left instead of inserting at the beginning on the right
                    None | Some(0) => {
                        // println!("recursive insert {}, {}: upper node: {:?}, going left", element_idx, node_idx, next_node);
                        went_left = true;
                        self.insert_helper(element_idx, node_idx * 2 + 1, el)
                    },
                    Some(delta) => {
                        // println!("recursive insert {}, {}: upper node: {:?}, going right", element_idx, node_idx, next_node);
                        self.insert_helper(delta, node_idx * 2 + 2, el)
                    },
                }
            },
            Some(Node::Leaf(v)) => {
                // println!("recursive insert {}, {}: leaf node with size: {}", element_idx, node_idx, v.len());
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
        // now current_leaf is None
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

    // #[test]
    // fn test_div_ceil() {
    //     let n: u64 = 0;
    //     assert_eq!(n.div_ceil(2), 1);
    // }

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
    fn test_insert() {
        for idx in 0..7 {
            let mut mixlist = MixList::new(&(0..7).collect::<Vec<i64>>(), 2);
        }
    }

    #[test]
    fn test_iterator() {
        let source_vec = (0..16).collect::<Vec<i64>>();
        let mixlist = MixList::new(&source_vec, 2);
        assert_eq!(mixlist.iter().map(|el| el.value).collect::<Vec<i64>>(),
                   source_vec);
    }

    // fails: 150, 25
    // divide by 25?
    // 6, 1
    // 12, 2?
    // #[test]
    // fn test_big_chunks() {
    //     let size = 10;
    //     let mixlist = MixList::new(&(0..size).collect::<Vec<i64>>(), 2);
    //     mixlist.print();
    //     for i in 0..size {
    //         assert_eq!(mixlist.get(i as usize), Some(&MixListElement::new(i as i64)));
    //     }
    // }
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
        // MixList::new only works for a power of 2 number of nodes. 5000 = 2**3 * 5**4
        625
    };
    let mut mixlist = MixList::new(&raw_vec, chunk_size);
    let mut idx = 0;
    let total_size = raw_vec.len();
    let size_but_one = total_size as i64 - 1;

    // println!("first element of mixlist: {:?}, first element of vec: {}", mixlist.get(0), raw_vec[0]);
    while idx < total_size {
        // println!("\n******* dumping list before remove call with args: {}  *******", idx);
        // mixlist.print();
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
        // println!("idx of mixlist: {}, existing el: {:?}, new el: {:?}, new_idx: {}", idx, el, new_el, new_idx);
        assert!(new_idx >= 1 && new_idx <= size_but_one as i64);
        // println!("\n******* dumping list before call with args: {}, {:?}  *******", new_idx, new_el);
        // mixlist.print();
        // println!("list looks like: {:?}, arguments to insert: {}, {:?}", mixlist, new_idx, new_el);
        mixlist.insert(new_idx as usize, new_el);
    }
    // mixlist.print();
    let idx = mixlist.iter().enumerate().find_map(|(idx, el)| if el.value == 0 { Some(idx) } else { None }).unwrap();
    println!("zero idx: {}", idx);
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
