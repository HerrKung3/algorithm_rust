use std::i32::MAX;

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = MAX;
        let mut max_profit = 0;

        // 可以循环一遍的原因是，一天只能买或卖。
        // 每次循环都在尝试当天是买入合适还是卖出合适。
        for p in prices.iter() {
            if *p < min_price {
                min_price = *p;
            } else if *p - min_price > max_profit {
                max_profit = *p - min_price
            }
        }

        max_profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];

    let res = Solution::max_profit(prices);

    println!("result: {}", res);
}
