pub struct Solution {}

// 下面是更符合时间复杂度要求的二分法：
// 题目有两个已知条件：
// 数组是有序的。
// 只有一个数出现一次。
// 第二个条件意味着，数组的长度一定是奇数。
// 第一个条件意味着，出现两次的数，必然相邻，
// 不可能出现 1,2,1 这样的顺序。
// 这也意味着，只出现一次的那个数，
// 一定位于偶数下标上。

// 这启发我们去检查偶数下标 2k。

// 示例 1 的 nums=[1,1,2,3,3,4,4,8,8]：
// 如果 nums[2k]=nums[2k+1]，
// 说明只出现一次的数的下标 >2k。
// 如果 nums[2k]≠nums[2k+1]，
// 说明只出现一次的数的下标 ≤2k。

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() / 2;

        while left < right {
            let mid = (right - left) / 2 + left;

            if nums[2 * mid] != nums[2 * mid + 1] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[2 * right]
    }
}

fn main() {
    let nums = vec![3, 3, 7, 7, 10, 11, 11];

    let res = Solution::single_non_duplicate(nums);

    println!("result: {}", res);
}
