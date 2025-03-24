pub struct Solution {}

// 有两种改法，第一种是将左边改成和右边的一样小，比如 4,2,3 -> 2,2,3
// 另一种改法是将右边改成和左边一样大，比如3,
// 4,2,3 -> 3,4,4,3, 如果用第一种改法3,4,2,3
// -> 3,2,2,3 那么会被误判成true。
// 我之前的做法是两种改法都尝试， 代码如下。
// func checkPossibility(nums []int) bool {
// 	changeTimes := 1
// 	if len(nums) <= 2 {
// 		return true
// 	}
// 	for i := 0; i < len(nums)-1; i++ {
// 		if nums[i] > nums[i+1] && changeTimes >=
// 0 { 			changeTimes--
// 			backUp := nums[i+1]
// 			nums[i+1] = nums[i]
// 			if !sort.IntsAreSorted(nums) {
// 				nums[i], nums[i+1] = backUp, backUp
// 				return sort.IntsAreSorted(nums)
// 			}
// 		}
// 	}
// 	return changeTimes >= 0
// }

impl Solution {
    pub fn check_possibility1(mut nums: Vec<i32>) -> bool {
        if nums.len() <= 2 {
            return true;
        }

        // 记录修改次数
        let mut change_times = 1;

        for i in 0..nums.len() - 1 {
            if i == 0
                && nums[i] > nums[i + 1]
                && change_times >= 0
            {
                change_times -= 1;
                nums[i] = nums[i + 1];
                continue;
            }

            if nums[i] > nums[i + 1] && change_times >= 0 {
                change_times -= 1;
                let temp = nums[i];
                // 先尝试将左边的改小
                nums[i] = nums[i + 1];
                // 判断第一次尝试之后的数组是否满足要求,
                // 不满足则第二次尝试
                if !is_non_decreasing(&nums[(i - 1)..]) {
                    nums[i] = temp;
                    nums[i + 1] = temp;
                    return is_non_decreasing(&nums[(i - 1)..])
                        && change_times >= 0;
                }
            }
        }
        return change_times >= 0;
    }
}

fn is_non_decreasing(nums: &[i32]) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    nums.windows(2).all(|pair| pair[0] <= pair[1])
}

fn main() {
    // let nums = vec![3, 4, 2, 3];
    let nums = vec![4, 2, 1];

    let res = Solution::check_possibility1(nums);

    println!("result: {}", res);
}
