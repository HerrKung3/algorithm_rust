pub struct Solution {}

// 异或运算
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for &num in nums.iter() {
            res = res ^ num;
        }

        return res;
    }
}

fn main() {
    let nums = vec![3, 3, 7, 7, 10, 11, 11];

    let res = Solution::single_non_duplicate(nums);

    println!("result: {}", res);
}
