use std::io;
fn main() {
    println!("猜数游戏!");


    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数字是:{}", guess);
}
