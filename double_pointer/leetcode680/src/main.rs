pub struct Solution{}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.into_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return  helper(&s, left+1, right) || helper(&s, left, right-1)
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}

fn helper(s: &[u8], mut start: usize, mut end: usize) -> bool {
    while start < end {
        if s[start] != s[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}

fn main() {
    let result = Solution::valid_palindrome(String::from("afbca"));
    println!("result = {}", result);
}