fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(10);
    print_labeled_measurement(5, 'h');
    statement_expression();
    let x = five();
    println!("函式 five 回傳的數值為：{x}");
    let y = plus_one(5);
    println!("函式 plus_one 回傳的數值為：{y}");
    let (a, b, c) = compute();
    println!("函式 compute 回傳的數值為：{a}, {b}, {c}");
    let result = calculate(10, 5);
    println!("函式 calculate 回傳的數值為：sum = {}, diff = {}, product = {}", result.sum, result.diff, result.product);
    let mut a = 0;
    let mut b = 0;
    fill_data(&mut a, &mut b);
    println!("函式 fill_data 修改後的數值為：a = {}, b = {}", a, b);
}

fn another_function() {
    println!("另一支函式。");
}
fn another_function2(x: i32) {
    println!("第二支函式。");
    println!("x 的數值為：{x}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("測量值爲：{value}{unit_label}");
}
fn statement_expression() {
    // 陳述式（Statements）是進行一些動作的指令，且不回傳任何數值。
    // 表達式（Expressions）則是計算並產生數值。讓我們來看一些範例：
    let y = {
        let x = 3;
        x + 1 // 注意這裏沒有分號
    };
    // println!("x 的結果為：{x}");
    // the binding `x` is available in a different scope in the same function

    // let x = (let y = 6); 
    // 這會產生錯誤error: expected expression, found statement (`let`)
    // ，因爲 let 陳述式不會回傳值

    println!("y 的數值為：{y}");
}
fn five() -> i32 {
    // 在 five 函式中沒有任何函式呼叫、巨集甚至是 let 陳述式，只有一個 5。這在 Rust 中完全是合理的函式。請注意到函式的回傳型別也有指明，就是 -> i32。
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // 這會產生錯誤 error: expected `i32`, found `()`
}
fn compute() -> (i32, i32, i32) {
    let a = 10;
    let b = 20;
    let c = 30;
    (a, b, c)
}
struct ResultData {
    sum: i32,
    diff: i32,
    product: i32,
}

fn calculate(a: i32, b: i32) -> ResultData {
    ResultData {
        sum: a + b,
        diff: a - b,
        product: a * b,
    }
}
fn fill_data(a: &mut i32, b: &mut i32) {
    *a = 10;
    *b = 20;
}
