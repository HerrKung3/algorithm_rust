pub struct Solution;

const VOWELS: [u8; 10] = [b'A', b'E', b'I', b'O', b'U', b'a', b'e', b'i', b'o', b'u'];

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut left: isize = 0;
        let mut right:isize = (s.len() - 1).try_into().unwrap();

        while left < right {
            // 在此处创建变量，简化后续判断的表达式，但变量只会在外层while的每次循环中被赋值一次。
            // let contain_left = VOWELS.contains(&s[left as usize]);
            // let contain_right = VOWELS.contains(&s[right as usize]);

            // while !contain_left && (left < right) {
            //     left += 1;
            // }

            // while !contain_right && (left < right) {
            //     right -= 1;
            // }

            while !VOWELS.contains(&s[left as usize]) && (left < right) {
                // println!("left = {}", left);
                left += 1;
            }

            while !VOWELS.contains(&s[right as usize]) && (left < right) {
                // println!("right = {}", right);
                right -= 1;
            }

            // Vec<>的index类型是usize，如果left = 0, right = 0
            // right -= 1后会导致overflow
            s.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }
        String::from_utf8(s).unwrap()
    }
}

fn main() {
    let s = String::from("IceCreAm");
    let result = Solution::reverse_vowels(s);
    println!("result = {:?}", result);
}