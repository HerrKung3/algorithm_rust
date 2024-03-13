use a0_stack::Stack;

//将num从十制转换到base进制
fn base_convert(mut num: usize, base: usize) -> String {
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

fn main() {
    let num1 = 9;
    let base1 = 2;
    let res1 = base_convert(num1, base1);
    print!("10进制的{}转换成{}进制为: {}\n",num1, base1, res1);

    let num2 = 12;
    let base2 = 16;
    let res2 = base_convert(num2, base2);
    print!("10进制的{}转换成{}进制为: {}\n",num2, base2, res2);
}
