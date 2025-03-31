pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(
        letters: Vec<char>,
        target: char,
    ) -> char {
        // 修枝， 避免不必要的二分。
        if letters[letters.len() - 1] <= target
            || letters[0] > target
        {
            return letters[0];
        };

        let mut left = 0;
        let mut right = letters.len() - 1;

        let mut res = letters[0];
        while left <= right {
            let mid = left + (right - left) / 2;
            if letters[mid] <= target {
                left = mid + 1;
            } else {
                res = letters[mid];
                right = mid - 1;
            }
        }

        res
    }
}

fn main() {
    let letters = vec!['b', 'c', 'e', 'e', 'g', 'p', 'y'];
    let target = 'c';

    let res = Solution::next_greatest_letter(letters, target);

    println!("result: {}", res);
}
