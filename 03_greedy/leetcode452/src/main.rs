pub struct Solution {}

// 确定一个区域A中所有线段的公共区域，
// 该区域的右边界（pos）就是能满足引爆区域A中所有气球(线段)
// 且最容易让下一个线段引爆的位置。
impl Solution {
    pub fn find_min_arrow_shots(
        mut points: Vec<Vec<i32>>,
    ) -> i32 {
        // 按照线段的左边界排序
        points.sort();

        let mut cnt = 1;
        let mut pos = points[0][1];

        for point in points.iter().skip(1) {
            // 如果下一个线段的左边界没有超出公共区域的右边界
            // 说明无法使用一支箭将前面区域的所有线段引爆且
            // 同时引爆该线段，cnt+=1，并且新增一个区域，
            // 并计算该区域的公共区域右边界pos的初始值。
            if point[0] > pos {
                pos = point[1];
                cnt += 1;
            } else {
                // 如果下一个线段的左边界超过了pos，
                // 则证明该线段与前面线段的公共区域仍然有交集，
                // 更新pos即可。
                pos = pos.min(point[1])
            }
        }

        cnt
    }
}

fn main() {
    let points =
        vec![vec![10, 16], vec![2, 7], vec![1, 6], vec![7, 12]];
    let res = Solution::find_min_arrow_shots(points);

    println!("result: {}", res);
}
