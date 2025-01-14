pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnt = [0; 3];

        for num in nums.iter() {
            cnt[*num as usize] += 1;
        }
        *nums = (0..3)
            .flat_map(|n| {
                std::iter::repeat(n).take(cnt[n as usize])
            })
            .collect();
    }

    pub fn sort_colors1(nums: &mut Vec<i32>) {
        let target = 1;
        let mut less = 0;
        let mut greater = nums.len();
        let mut i = 0;

        // let mut greater = nums.len() - 1; while
        // i <= greater
        //...
        // nums.swap(i, greater);
        // 如果这么写，
        // 当nums = [2, 2]时，greater = 0
        // 也是可以进入到while循环的，
        // 因为始终i=0循环中，greater -= 1；
        // 导致溢出。这种循环中-1的，
        // 都应该避免=0时进入循环。
        while i < greater {
            if nums[i] < target {
                nums.swap(i, less);
                less += 1;
                i += 1
            } else if nums[i] > target {
                nums.swap(i, greater - 1);
                greater -= 1;
            } else {
                i += 1;
            }
        }
    }
}

fn main() {
    // [0 0 1 1 2 2 2 2]
    let mut nums = vec![2];

    Solution::sort_colors1(&mut nums);

    println!("result: {:?}", nums);
}
