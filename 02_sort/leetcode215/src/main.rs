use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn bubble_sort<T: Ord>(nums: &mut Vec<T>) {
        let n = nums.len();

        // 外层循环，控制总循环次数，每次找到一个未被固定的最大的数字，固定到数组最末尾。
        for i in 0..n {
            // 内层循环，控制比较范围，外层每次循环都是找到一个违背固定的最大数值固定末尾，
            // 因此内层循环要避开比较固定的数值。
            for j in 0..n-i-1 {
                if nums[j] > nums[j+1] {
                    nums.swap(j, j+1);
                }
            }
        }
    }

    pub fn select_sort<T: Ord>(nums: &mut Vec<T>) {
        let n = nums.len();

        // 外层循环，控制总循环次数，每次找到一个未被固定的最小数值的index，然后和固定区的边界index交换。
        for i in 0..n {
            let mut min_index = i;
            
            // 在未固定区域中找到最小值的index
            for j in i+1..n {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }
            // 和固定区边界交换
            nums.swap(i, min_index);
        }
    }

    pub fn insert_sort<T: Ord + Clone + Copy>(nums: &mut Vec<T>) {
        let n = nums.len();

        for i in 1..n {
            let cur = nums[i];
            let mut j = i;
            // 和前一个元素比较，如果小于前一个元素就把前一个元素复制到后一个元素，为后面的“插入”腾挪位置。
            // 不是真正的插入，而是大于cur的元素都复制了一遍，覆盖了后面的位置，
            // 最后一个大于cur的元素在while退出时，复制了两次，所以在退出后，可以把该元素的位置覆盖cur。
            // 使用真正的insert，会导致nums长度增加，当len > cap时，nums在heap上的整体转移将耗费性能。
            // 但每个元素最多只会insert一次，因此长度最多x2，对于每次cap扩容一倍的语言，只会发生一次整体迁移。
            // 但使用insert最大的问题在于insert操作会破坏最外层的for循环。
            while j > 0 && cur < nums[j-1] {
                nums[j] = nums[j-1];
                j -= 1;
            }
            nums[j] = cur;
        }
    }

    //              root
    //            /       \
    //          A           B
    //        /   \       /   \
    //      C       D   E       F
    // 递归压栈顺序ACDBEF
    // 处理顺序CDAEFBroot，类似于后序遍历。
    pub fn merge_sort(nums: &mut [i32]) {
        let mid = nums.len() / 2;
        if mid == 0 {
            return;
        }

        Self::merge_sort(&mut nums[..mid]);
        Self::merge_sort(&mut nums[mid..]);

        // 停止递归并第一次返回后，left和right都是长度=1的切片。
        // 先对nums的left和right两部分排序，结果保存到ret中。
        let mut ret = nums.to_vec();

        Self::merge(&nums[..mid], &nums[mid..], &mut ret[..]);

        nums.copy_from_slice(&ret);
    }

    fn merge(left: &[i32], right: &[i32], ret: &mut [i32]) {
        // println!("left length = {}, left = {:?}", left.len(), left);
        // println!("right length = {}, right = {:?}", right.len(), right);
        // println!("");
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                ret[k] = left[i];
                k += 1;
                i += 1;
            }else {
                ret[k] = right[j];
                k += 1;
                j += 1;
            }
        }

        if i < left.len() {
            ret[k..].copy_from_slice(&left[i..]);
        }

        if j < right.len() {
            ret[k..].copy_from_slice(&right[j..]);
        }
    }

    pub fn quick_sort(nums: &mut [i32]) {
        // 递归终止条件
        if nums.len() <= 1 {
            return;
        }

        // 分区，[<, >=] pivot
        let pivot_index = Self::partition(nums);

        // 缩小规模继续分区
        let (left, right) = nums.split_at_mut(pivot_index);
        Self::quick_sort(left);
        Self::quick_sort(&mut right[1..]);
    }

    fn partition(nums: &mut [i32]) -> usize {
        let n = nums.len();
        let pivot = nums[n - 1];
        let mut i = 0;

        // 遍历除基准元素pivot外的所有元素
        for j in 0..nums.len() - 1 {
            if nums[j] < pivot {
                nums.swap(i, j);
                i += 1;
            }
        }

        // 退出for循环后的i处元素是大于pivot，彼此交换即可
        nums.swap(i, n-1);

        // 返回基准元素index
        i
    }

    pub fn heap_sort(nums: &mut [i32]) {
        let mut heap = BinaryHeap::new();

        for num in nums.iter() {
            // 转换成小根堆
            heap.push(Reverse(*num));
        }

        for num in nums.iter_mut() {
            *num = heap.pop().unwrap().0;
        }
    }
}

fn main() {
    let mut nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
    Solution::heap_sort(&mut nums);
    println!("sorted nums = {:?}", nums);
}