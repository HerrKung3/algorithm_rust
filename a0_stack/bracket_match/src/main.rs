use bracket_match::bracket_checker;

fn main() {
    let mut brackets = vec![];
    brackets.push("((){}[9+1])");
    brackets.push("([]{}()");
    brackets.push("({}[)])");
    brackets.push("())");
    brackets.push("())");
   
   for b in brackets {
    let result = bracket_checker(b);
    println!("括号匹配结果： {}", result)
   }
}
