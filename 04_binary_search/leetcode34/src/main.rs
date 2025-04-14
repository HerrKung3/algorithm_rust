pub struct Solution {}

// 实际上是找到两个target，分别是左右边界。
// 遇到的问题：
// 1.找右边界时，如果mid=l+(r-l)/2;
// nums = [5,7,7,8,8,10], target = 8
// 上面例子就会无法退出循环。
// 原因在于l=mid=4时，r=5永远无法向左移动。
// 2. nums[5,7,7,8,8,10], target = 10,
// 可能会出现边界溢出的问题，
// 原因在于在循环中l = m + 1，超出了边界。
// 解决办法：
// 1. 使mid=l+(r-l)/2+1,灵感乍现且有效的方法
// 归结于一种更合适的对称性，
// 寻找左右边界时逻辑上的对称性。
// 2. while left<=right改为left<right.

// 另一个方法是将寻找右边界问题转换成
// 寻找target+1的左边界问题。
// 找到后index - 1即可。
impl Solution {
    pub fn search_range(
        nums: Vec<i32>,
        target: i32,
    ) -> Vec<i32> {
        let mut res = vec![-1, -1];
        let n = nums.len();

        if n == 0 || nums[0] > target || nums[n - 1] < target {
            return res;
        }

        let mut left = 0;
        let mut right = n - 1;

        // 找到左边界
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        // 存储答案
        if nums[left] == target {
            res[0] = left as i32;
        }

        // 格式化
        left = 0;
        right = n - 1;

        // 找到右边界
        while left < right {
            let mid = left + (right - left) / 2 + 1;
            if nums[mid] > target {
                right = mid - 1;
            } else {
                left = mid;
            }
        }

        // 储存答案
        if nums[left] == target {
            res[1] = left as i32;
        }

        res
    }
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];

    let res = Solution::search_range(nums, 8);

    println!("rseult: {:?}", res);
}
