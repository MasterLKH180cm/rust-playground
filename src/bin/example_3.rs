fn main() {
    let x = 5;

    let x = x + 1;
    // Shadowing 範例
    {
        let x = x * 2;
        println!("x 在內部範圍的數值為：{x}");
    }

    println!("x 的數值為：{x}");

    
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces 的長度為：{spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // gets error: expected `&str`, found `usize`


}
