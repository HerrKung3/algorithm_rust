pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<usize> {
        // 记录每个字符最后出现的索引
        let mut last_indexes = vec![0; 26];

        let mut sub_array_length = vec![];

        for (i, b) in s.bytes().enumerate() {
            last_indexes[(b - b'a') as usize] = i;
        }

        let mut start = 0;
        let mut end = 0;
        // 遍历查看字符最后出现的索引，记录最大值
        // 遍历到最大值时，始终没有找到更大的，
        // 说明可以划分成子串。
        // 如果有，那就更新最大值。
        for (i, b) in s.bytes().enumerate() {
            let last_index = last_indexes[(b - b'a') as usize];
            if last_index > end {
                end = last_index;
            }

            if i == end {
                sub_array_length.push(end - start + 1);
                start = end + 1;
            }
        }

        sub_array_length
    }
}

fn main() {
    let s = "ababcbacadefegdehijhklij".to_string();

    let res = Solution::partition_labels(s);

    println!("result: {:?}", res);
}
