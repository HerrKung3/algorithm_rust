pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut right = x / 2 + 1;
        // 当x>=46340,
        // 其平方大于等于2^32-1，整数溢出。
        // 而输入空间为i32,
        // 因此其中的数字最大平方根就是46340.
        if x >= 46340 {
            right = 46340;
        }

        let mut left = 0;
        let mut res = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid * mid > x {
                right = mid - 1;
            } else {
                res = mid;
                left = mid + 1;
            }
        }
        res
    }
}

fn main() {
    let num = 2147395599;

    let res = Solution::my_sqrt(num);

    println!("result: {}", res);
}
