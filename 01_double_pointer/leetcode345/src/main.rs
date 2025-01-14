pub struct Solution;

const VOWELS: [u8; 10] = [
    b'A', b'E', b'I', b'O', b'U', b'a', b'e', b'i', b'o', b'u',
];

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            // 在此处创建变量，简化后续判断的表达式，
            // 但变量只会在外层while的每次循环中被赋值一次。
            // let contain_left = VOWELS.contains(&s[left as
            // usize]); let contain_right =
            // VOWELS.contains(&s[right as usize]);

            // while !contain_left && (left < right) {
            //     left += 1;
            // }

            // while !contain_right && (left < right) {
            //     right -= 1;
            // }

            while (left < right) && !VOWELS.contains(&s[left]) {
                left += 1;
            }

            while (left < right) && !VOWELS.contains(&s[right])
            {
                right -= 1;
            }

            // Vec<>的index类型是usize，如果left = 0, right = 0
            // right -= 1后会导致overflow
            // 一种方式是将left、right声明为isize，作为index时as
            // uszie 另一种方式是在此处先判断一下
            if left < right {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        String::from_utf8(s).unwrap()
    }
}

fn main() {
    let s = String::from("IceCreAm");
    let result = Solution::reverse_vowels(s);
    println!("result = {:?}", result);
}
