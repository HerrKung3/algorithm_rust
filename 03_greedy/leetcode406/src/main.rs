pub struct Solution {}

// 身高高的人看不见身高低的人
// 先按照身高h进行降序排序，根据题目要求，
// 身高矮的无论插入到什么位置都不会对之前插入的造成影响。
// 这样只需要将身高矮的插入到k个位置即可。
// 对于身高相同的，按照k升序排序，
// 因为按照题目要求，
// 身高相同的也会对k产生影响。
impl Solution {
    pub fn reconstruct_queue(
        mut people: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });

        // 拍完序之后，只需要将K放在第k位即可，
        // 因为后面元素的h小于前面元素的h。
        // 后面元素的插入，不会影响前面元素的k。
        // println!("sorted people: {:?}", people);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for p in people.iter() {
            res.insert(p[1] as usize, p.to_vec());
        }

        res
    }
}

fn main() {
    let people = vec![
        vec![7, 0],
        vec![4, 4],
        vec![7, 1],
        vec![5, 0],
        vec![6, 1],
        vec![5, 2],
    ];

    let res = Solution::reconstruct_queue(people);
    println!("result: {:?}", res);
}
