fn main() {
    println!("條件判斷範例：");
    if_else_example();
    let_if_expression();
    loop_example();
    while_example();
    for_example();
}
fn if_else_example() {
    let number = 6;

    if number % 4 == 0 {
        println!("數字能被 4 整除");
    } else if number % 3 == 0 {
        println!("數字能被 3 整除");
    } else if number % 2 == 0 {
        println!("數字能被 2 整除");
    } else {
        println!("數字不能被 4、3 或 2 整除");
    }
//     let number = 3;

//     if number { // 這會產生錯誤 error: mismatched types
//         println!("數字為三");
//     }

}

fn let_if_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // let number = if condition { 5 } else { "六" };
    // 這會產生錯誤 error: if and else have incompatible types

    println!("數字的值為：{number}");
}

fn loop_example() {
    let mut count = 0;
    // counting_up 標籤
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("陣列元素的值為：{element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}