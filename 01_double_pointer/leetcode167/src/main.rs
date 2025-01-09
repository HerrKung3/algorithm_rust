use std::cmp::Ordering;

pub struct Solution {

}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        let mut result = vec![];

        while i < j {
            match target.cmp(&(numbers[i] + numbers [j])) {
                Ordering::Less => {j -= 1;}
                Ordering::Greater => {i += 1;}
                Ordering::Equal => {
                    result.push(i as i32 + 1);
                    result.push(j as i32 + 1);
                    break;
                }
            }
        }
        result
    }
}

fn main() {
    let numbers = vec![3,24,50,79,88,150,345];
    let result = Solution::two_sum(numbers, 200);
    println!("result = {:?}", result);
}
