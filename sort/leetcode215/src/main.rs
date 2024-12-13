pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_by(|a, b| {
            b.cmp(a)
        });

        nums[k as usize - 1]
    }
}

fn main() {
    let nums = vec![3,2,3,1,2,4,5,5,6];
    let kth_largest = Solution::find_kth_largest(nums, 4);

    println!("kth_largest = {}", kth_largest);
}
