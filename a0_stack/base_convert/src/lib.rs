use a0_stack::Stack;

//将num从十制转换到base进制
pub fn base_convert(mut num: usize, base: usize) -> String {
    let digits = ['0', '1','2','3','4','5','6','7','8','9','A','B','C','D','E','F',];
    let mut rem_stack = Stack::new();

    while num > 0 {
        let rem = num % base;
        rem_stack.push(rem);
        num /= base;
    }

    let mut result = String::from("");
    while !rem_stack.is_empty() {
        let digit = rem_stack.pop().unwrap() as usize;
        result.push(digits[digit]);
    }

    return result
}