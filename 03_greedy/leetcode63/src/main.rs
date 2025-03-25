pub struct Solution {}

// 可以这样理解：按照一定的规律将数组划分。
// 子数组后面的数不会将子数组内部数字的和消耗(增加)为负，就可以一直累加。
// 如果消耗成负，那么应该记录消耗（增加）过程中的最大和，然后寻找下一个片段。
// 数组划分成子数组是按内部之和是否降为0.
// 如果一个数组遍历完，和始终为正，
// 那么就只有一个子数组。
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut all_sub_array_max_sums = vec![];

        let mut cur_sub_array_max_sum = nums[0];
        let mut cur_sub_array_sum = nums[0];
        for i in 0..nums.len() - 1 {
            // cur_sub_array_sum <= 0
            // 说明即将进入下一个子数组
            if cur_sub_array_sum <= 0 {
                // 记录当前子数组的最大和
                all_sub_array_max_sums
                    .push(cur_sub_array_max_sum);

                // 清空记录，用来记录下一个子数组
                cur_sub_array_sum = nums[i + 1];
                cur_sub_array_max_sum = nums[i + 1];
            } else {
                // cur_sub_array_sum > 0
                // 说明还在同一个子数组中
                cur_sub_array_sum += nums[i + 1];
                if cur_sub_array_sum > cur_sub_array_max_sum {
                    cur_sub_array_max_sum = cur_sub_array_sum;
                }
            }
        }
        all_sub_array_max_sums.push(cur_sub_array_max_sum);

        let mut max_sum = all_sub_array_max_sums[0];
        for i in all_sub_array_max_sums.iter() {
            if max_sum < *i {
                max_sum = *i;
            }
        }
        max_sum
    }
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];

    let res = Solution::max_sub_array(nums);

    println!("result: {res}")
}
