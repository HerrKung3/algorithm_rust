pub struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        let s = s.as_bytes();
        // 记录t中有s的字符数量
        let mut cnt = 0;
        for &c in t.as_bytes().iter() {
            if cnt == s.len() {
                return true;
            }

            if c == s[cnt] {
                cnt += 1;
            }
        }

        cnt == s.len()
    }
}

fn main() {
    let s = "".to_string();
    let t = "abc".to_string();

    let res = Solution::is_subsequence(s, t);

    println!("result: {}", res);
}
