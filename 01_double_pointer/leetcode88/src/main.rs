pub struct Solution {}

impl Solution {
    pub fn merge(
        nums1: &mut Vec<i32>,
        m: i32,
        nums2: &mut Vec<i32>,
        n: i32,
    ) {
        let mut m = m as usize;
        let mut n = n as usize;
        // let mut tail1 = m - 1;
        // let mut tail2 = n - 1;
        let mut insert_index = m + n - 1;

        // while tail2>=0会导致overflow，
        // 因为tail2=0也可以进入循环，而在循环中会对tail2-1.
        // while tail2>0的判断条件始终无法将nums[tail2=0]排序进去.
        // 可以直接使用m>0判断，在循环体中使用nums2[m-1]
        // while tail2 > 0 {
        //     match nums1[tail1].cmp(&nums2[tail2]) {
        //         Ordering::Greater | Ordering::Equal => {
        //             nums1.swap(insert_index, tail1);
        //             if tail1 > 0 {
        //                 tail1 -= 1;
        //             }
        //         }
        //         Ordering::Less  => {
        //             nums1[insert_index] = nums2[tail2];
        //             if tail2 > 0{
        //                 tail2 -= 1;
        //             }
        //         }
        //     }
        //     insert_index -= 1;
        // }

        while n > 0 {
            if m == 0 || nums1[m - 1] < nums2[n - 1] {
                nums1[insert_index] = nums2[n - 1];
                if n > 0 {
                    n -= 1
                }
            } else {
                nums1.swap(m - 1, insert_index);
                if m > 0 {
                    m -= 1
                }
            }
            insert_index -= 1;
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("merged sequence = {:?}", nums1);
}
