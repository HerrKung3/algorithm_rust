use a2_dequeue::Deque;

//检查回文字符
//出队 <- radar -> 出队
pub fn pal_checker(pal: &str) -> bool {
    let mut deque = Deque::new(pal.len());

    for c in pal.chars() {
        _ = deque.add_front(c);
    }

    for _i in 0..pal.len()/2 {
        if deque.remove_front() != deque.remove_rear() {
            return false;
        }
    }
    
    true
}

//判断一个字符串能否在删除有限个字符后变成回文结构？
//ABBCA -> ABBA, ABCDEFA -> ADA
//pal:带待判断字符串， num:允许删除次数
//这种情况用双指针要更方便一些
pub fn pal_checker1(pal: &str, num: isize) -> bool {
    let chars: Vec<char> = pal.chars().collect();
    helper(chars, num)
}

fn helper(chars: Vec<char>, num: isize) -> bool {
    if num < 0 {
        return false;
    }

    let mut left = 0;
    let mut right = chars.len() - 1;
    
    while left < right {
        if chars[left] != chars[right] {
            return helper(chars[left+1.. right+1].to_vec(), num-1) || helper(chars[left..right].to_vec(), num - 1);
        }else {
            left += 1;
            right -= 1;
        }
    }

    true
}