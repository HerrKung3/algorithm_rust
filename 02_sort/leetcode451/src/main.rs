use std::{
    collections::{BinaryHeap, HashMap},
    iter::repeat,
};

pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut frequent_map: HashMap<u8, usize> =
            HashMap::new();
        let mut res = "".to_string();

        for c in s.into_bytes().into_iter() {
            frequent_map
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut frequent_heap: BinaryHeap<(usize, u8)> =
            BinaryHeap::new();
        for k_v in frequent_map.into_iter() {
            frequent_heap.push((k_v.1, k_v.0));
        }

        while !frequent_heap.is_empty() {
            let ele = frequent_heap.pop().unwrap();
            res.extend(repeat(ele.1 as char).take(ele.0));
        }

        res
    }
}

fn main() {
    let s = "AabbaaB".to_string();
    let res = Solution::frequency_sort(s);

    println!("result: {:?}", res);
}
