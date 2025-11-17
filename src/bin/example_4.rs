use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("請輸入陣列索引");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("讀行失敗");

    let index: usize = index
        .trim()
        .parse()
        .expect("輸入的索引並非數字");

    let element = a[index];
    // 輸入超出陣列長度的數值
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    println!(
        "索引 {index} 元素的數值爲：{element}"
    );
}
