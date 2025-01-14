pub struct Solution {}

// 应该将尽可能短的线段留下，为了比较线段长度，
// 尤其是有重叠区域的，需要定义一种比较规则，
// 将其放在一起，比如[1, 2], [1, 3], [2,
// 3]是按线段起始位置大小排序的。
impl Solution {
    pub fn erase_overlap_intervals(
        mut intervals: Vec<Vec<i32>>,
    ) -> i32 {
        // intervals.sort_by(|a, b| {
        //     if a[0] == b[0] {
        //         a[1].cmp(&b[1])
        //     }else {
        //         a[0].cmp(&b[0])
        //     }
        // });
        intervals.sort_unstable();

        // println!("intervals: {:?}", intervals);
        let mut cnt = 0;
        let mut end = intervals[0][1];

        for interval in intervals.iter().skip(1) {
            if interval[0] >= end {
                // start = interval[0];
                end = interval[1];
            } else {
                cnt += 1;
                // 如果有重叠，则末尾越短越好
                end = end.min(interval[1])
            }
        }

        cnt
    }
}

fn main() {
    let intervals =
        vec![vec![1, 3], vec![1, 2], vec![2, 3], vec![3, 4]];
    let res = Solution::erase_overlap_intervals(intervals);
    println!("result: {}", res);
}
