use a0_stack::Stack;

//检查左右括号是否匹配
fn bracket_match(left: char, right: char) -> bool {
    let lefts = "([{";
    let rights = ")]}";

    lefts.find(left) == rights.find(right)
}

pub fn bracket_checker(brackets: &str) -> bool {
    let mut stack = Stack::new();
    for bracket in brackets.chars() {
        match bracket {
            '(' | '[' | '{' => {
                stack.push(bracket);
            }

            ')' | ']' | '}' => {
                let left = if let Some(left) = stack.pop() {
                    left
                }else {
                    return false;
                };

                if bracket_match(left, bracket) == false {
                    return false;
                }
            }

            _ => continue
        }
    }

    stack.is_empty()
}