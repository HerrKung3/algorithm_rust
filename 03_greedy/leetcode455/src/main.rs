pub struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut cnt = 0;
        let mut i = 0;
        let mut j = 0;

        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                cnt += 1;
                i += 1;
            }
            j += 1;
        }

        cnt
    }
}

fn main() {
    let g = vec![1, 2, 3];
    let s = vec![1, 1];

    let res = Solution::find_content_children(g, s);
    println!("result: {}", res);
}