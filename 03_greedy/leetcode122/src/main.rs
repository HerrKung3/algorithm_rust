use std::i32::MAX;

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_porfit = 0;
        let mut min_price = MAX;

        for p in prices.iter() {
            if *p < min_price {
                min_price = *p
            } else if *p - min_price > 0 {
                max_porfit += *p - min_price;
                min_price = *p;
            }
        }

        max_porfit
    }
}

fn main() {
    // let prices = vec![7, 1, 5, 3, 6, 4];
    let prices = vec![1, 2, 3, 4, 5];

    let res = Solution::max_profit(prices);

    println!("result: {}", res);
}
