use expression_calculate::{infix_to_postfix, postfix_eval};

fn main() {
    let infix = "( 1 + 2 ) * ( 4 - 2 )";
    let postfix = if let Some(postfix) = infix_to_postfix(infix) {
        postfix
    }else {
        panic!("Invalid infix format")
    };

    let res = postfix_eval(&postfix);
    match res {
        Some(val) => {
            print!("{} = {:?}", infix, val)
        }
        None => {
            println!("Invalid infix format")
        }
    }

}
