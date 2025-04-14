pub struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(
        expression: String,
    ) -> Vec<i32> {
        let mut ret = Vec::new();
        for (i, ch) in (0..).zip(expression.chars()) {
            if ch != '+' && ch != '-' && ch != '*' {
                continue;
            }
            let (first, second) = (
                Solution::diff_ways_to_compute(
                    (&expression[0..i]).to_string(),
                ),
                Solution::diff_ways_to_compute(
                    (&expression[i + 1..]).to_string(),
                ),
            );
            for x in first.iter() {
                for y in second.iter() {
                    match ch {
                        '+' => ret.push(x + y),
                        '-' => ret.push(x - y),
                        '*' => ret.push(x * y),
                        _ => {}
                    }
                }
            }
        }
        if ret.is_empty() {
            ret.push(expression.parse::<i32>().unwrap());
        }
        ret
    }
}

fn main() {
    let expression = "2*3-4*5".to_string();

    let res = Solution::diff_ways_to_compute(expression);

    println!("result: {:?}", res)
}
