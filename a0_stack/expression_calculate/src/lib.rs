use std::collections::HashMap;
use a0_stack::Stack;
use bracket_match::bracket_checker;

//中缀表达式转后缀表达式
//(A * B) + (C * D) => AB*CD*+
pub fn infix_to_postfix(infix: &str) -> Option<String> {
    //检查括号是否匹配
    if !bracket_checker(infix) {
        return None;
    }

    //设置符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    //ops保存操作符号，postfix保存后缀表达式
    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        //操作数转换过程中前后顺序不变，直接放入postfix
        if ("A" <= token && token <= "Z") ||  ("0" <= token && token <= "9") {
            postfix.push(token);
        }else if token == "(" {
            //如果是左括号，直接压入操作符栈中
            op_stack.push(token)
        }else if token == ")" {
            //如果是右括号，则说明操作符栈中最顶端的括号优先级较高，需要弹出然后加入到postfix中，直到栈中的对应左括号弹出
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        }else {
            //没有括号，则操作符优先级保持默认，+ -小于* /
            //不断比较操作符优先级，优先级高的直接加入postfix，优先级低的入栈
            while !op_stack.is_empty() && prec[token] <= prec[op_stack.peek().unwrap()] {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }

    //将剩余的操作符加入postfix
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }
    
    //将postfix转成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str.push_str(c);
        postfix_str.push_str(" ");
    }
    
    Some(postfix_str)
}

//计算后缀表达式
pub fn postfix_eval(postfix: &str) ->Option<i32> {
    //少于五个字符则非法(加上两个空格)
    if postfix.len() < 5 {
        return None;
    }

    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            op_stack.push(token.parse::<i32>().unwrap());
        }else {
            //如果toekn是操作符，则弹出两个操作数
            //- /操作符对于操作数有顺序要求
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            let res = do_cal(token, op1, op2);
            op_stack.push(res);
        }
    }
    Some(op_stack.pop().unwrap())
}

fn do_cal(op: &str, op1: i32, op2: i32) -> i32 {
    match op {
        "+" => {
            op1 + op2
        }
        "-" => {
            op1 - op2
        }
        "*" => {
            op1 * op2
        }
        "/" => {
            if op2 == 0 {
                panic!("ZeroDivsionError: Invalid operation!")
            }
            op1 / op2
        }
        _ => {
            panic!("OperatorFormatError: Invalid operator!")
        }
    }
}