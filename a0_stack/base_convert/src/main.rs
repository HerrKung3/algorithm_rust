use base_convert::base_convert;

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
