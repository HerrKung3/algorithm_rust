pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(
        flowerbed: Vec<i32>,
        n: i32,
    ) -> bool {
        let n = n as usize;
        let mut cnt = 0;
        let mut num = 0;
        let l = flowerbed.len();

        let index = flowerbed.iter().position(|&x| x == 1);

        if let Some(index) = index {
            // 计算第一个1之前，可以种多少花
            num += index / 2;
            // 计算第一个1和最后一个1之间能种多少花
            for i in index + 1..l {
                if flowerbed[i] == 0 {
                    cnt += 1;
                } else {
                    num += (cnt - 1) / 2;
                    cnt = 0;
                }
            }
        } else {
            // 如果flowerbed全是0
            return (l + 1) / 2 >= n;
        }
        // 计算最后一个1之后，可以种多少花
        num += cnt / 2;

        num >= n
    }
}

fn main() {
    let flowerbed = vec![0, 0, 1, 0, 0, 0, 1, 0, 0];

    let res = Solution::can_place_flowers(flowerbed, 4);

    println!("result: {}", res);
}
