pub struct Solution {}

// 			        _
// 		        _
// 	        _
//      _
// 						        _
// 					        _
// 				        _

// 一个升序数组经过旋转后，会变成上图所示，
// 我们应该想办法找到右边线段的最左侧。
// 如果mid落在左边线段上，就有:
// nums[mid] ≥ nums[left] 大于等于nums[right]
// 此时应该把搜索范围控制在mid+1,right之间。
// （已经知道mid在左边，因此是从mid+1开始）。

// 如果循环条件是left <= right：
// 倒数第二次遍历时，left和mid是左边最高点。
// right是右边最低点。此时进入if分支，
// left = mid + 1后 = right。
// 此刻还能最后遍历一次，
// 最终left = right = mid = 最低点。

// 根据上面的分析，倒数第二次遍历时，
// right就已经等于最低点了。
// 因此可以把循环条件改成，
// while left < right。
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] >= nums[left]
                && nums[left] >= nums[right]
            {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[right]
    }
}

fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];

    let res = Solution::find_min(nums);

    println!("result: {}", res);
}
