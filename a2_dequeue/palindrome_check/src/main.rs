use palindrome_check::{pal_checker, pal_checker1};

fn main() {
    let pal1 = "racdbar";
    let res1 = pal_checker(pal1);
    println!("{} is palindrome? {}", pal1, res1);

    let res2 = pal_checker1(pal1,2);
    println!("{} is palindrome? {}", pal1, res2)
}
