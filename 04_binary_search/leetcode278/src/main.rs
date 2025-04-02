// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
pub struct Solution {}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;

        let mut res = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            if self.is_bad_version(mid) {
                res = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        res
    }

    pub fn is_bad_version(&self, n: i32) -> bool {
        n == 4
    }
}

fn main() {
    let n = 5;

    let solution = Solution {};

    let res = solution.first_bad_version(n);

    println!("result: {}", res);
}
