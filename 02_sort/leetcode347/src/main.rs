use std::collections::{BinaryHeap, HashMap};

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // key: number, value: frequent
        let mut frequent_map: HashMap<i32, i32> =
            HashMap::new();
        let mut res = vec![];

        for num in nums.into_iter() {
            frequent_map
                .entry(num)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let mut frequent_heap = BinaryHeap::new();

        for k_v in frequent_map.into_iter() {
            frequent_heap.push((k_v.1, k_v.0));
        }

        for _i in 0..k {
            let k_v = frequent_heap.pop().unwrap();
            res.push(k_v.1);
        }

        res
    }
}

fn main() {
    let nums = vec![2, 2, 3, 2, 4, 2, 4];
    let k = 2;
    let res = Solution::top_k_frequent(nums, k);

    println!("top {} frequent nums: {:?}", k, res);
}
