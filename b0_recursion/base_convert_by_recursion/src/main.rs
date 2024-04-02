const BASESTR: [&str; 16] = ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];

fn num2str_sec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    }else {
        num2str_sec(num/base, base) + BASESTR[(num%base) as usize]
    }
}

fn main() {
    let num = 100;
    let sb = num2str_sec(num, 2);
    let so = num2str_sec(num, 8);
    let sh = num2str_sec(num, 16);

    println!("{} is b{}, o{}, x{}", num, sb, so, sh)
}
