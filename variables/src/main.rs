fn main() {
    // let x = 5; <= immutable
    // mutableにしなければ代入できない！
    let mut x = 5;
    println!("The value o fx is: {}", x);
    x = 6;
    println!("The value o fx is: {}", x);

    // 定数にはmutキーワードは使用できない
    const MAX_POINTS: u32 = 100_000;
    println!("The value o fx is: {}", MAX_POINTS);

    // シャドーイングでletキーワードを重ねることで覆い隠せる
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value o fx is: {}", x); // <= 12

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value o fx is: {}", spaces);
}
