pub struct Solution {}

impl Solution {
    pub fn find_longest_word(
        s: String,
        mut dictionary: Vec<String>,
    ) -> String {
        // 先排序
        dictionary.sort_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                b.len().cmp(&a.len())
            }
        });

        // let mut longest: Vec<u8> = vec![];
        let s = s.into_bytes();
        for word in dictionary.into_iter() {
            let word = word.into_bytes();
            let mut p = 0;

            // 修枝
            if word.len() > s.len() {
                continue;
            }

            for str in s.iter() {
                if p < word.len() && *str == word[p] {
                    p += 1;
                }
            }

            // 第一个包含于s中的word，满足要求直接返回
            if p == word.len() {
                return String::from_utf8(word).unwrap();
            }
        }

        "".to_string()
    }
}

fn main() {
    let s = "abpcplea".to_string();
    let dictionary = vec![
        "ale".to_string(),
        "apple".to_string(),
        "monkey".to_string(),
        "plea".to_string(),
    ];
    let result = Solution::find_longest_word(s, dictionary);
    println!("longest word = {}", result);
}
