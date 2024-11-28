use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let target = c as usize;
        let mut right = (c as f64).sqrt().floor() as usize;
        let mut left = 0;
        while left <= right {
            let power_sum = left * left + right * right;
            match target.cmp(&power_sum) {
                Ordering::Greater => {
                    left += 1;
                }
                Ordering::Less => {
                    right -= 1;
                }
                Ordering::Equal => {
                    return true;
                }
            }
        }
        return false;
    }
}

fn main() {
    let result = Solution::judge_square_sum(2147483600);
    println!("result = {:?}", result);
}
