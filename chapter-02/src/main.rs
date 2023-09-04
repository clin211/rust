use std::io;

fn main() {
    println!("Hello, world!");
    println!("Please input a number");

    let mut input = String::new(); // 创建一个空字符串用来接收用户输入
    io::stdin()
        .read_line(&mut input) // 读取一行
        .expect("Failed to read line"); // 比较粗暴的错误处理
    println!("Your raw input is: {:?}.", input);
    let number: i64 = input
        .trim() // trim 去掉字符串前后的空格、换行
        .parse() // 将字符串转换为数字
        .expect("Please type a number!");
    println!("You typed: {}", number);
}
