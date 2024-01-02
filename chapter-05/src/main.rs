fn main() {
    greet();
    say_hello("Forest".to_string());
    println!("Sum: {}", sum(1, 2));
    println!("sub: {}", sub(12, 2));
    five_or_six();
    weather();

    create_loop();

    println!("return value: {}", create_loop_return_value());

    create_loop_and_break_label();
}

fn greet() {
    println!("Welcome to Rust!");
}

fn say_hello(name: String) {
    println!("Hello, {}!", name);
}

fn sum(a: i32, b: i32) -> i32 {
    // 表达式最后不能有分号，一旦加上了分号，就会变成一个语句，而语句没有返回值
    a + b
}

// 也可以使用 return 关键字来返回
fn sub(a: i32, b: i32) -> i32 {
    let result = a - b;
    return result;
}

// 控制流
fn five_or_six() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn weather() {
    let temperature = 20;

    let weather = if temperature >= 25 { "hot" } else { "cool" };

    println!("The weather today is {}.", weather);
}

// 循环

/**
 * loop 创建一个无限循环
 */
fn create_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter < 5 {
            continue;
        }
        println!("hello world!");

        if counter >= 5 {
            break;
        }
    }
}

fn create_loop_return_value() -> i32 {
    let target = 10;
    let mut sum = 0;
    let mut counter = 1;

    // 返回值类型
    let result = loop {
        sum += counter;

        if sum >= target {
            break counter; // The value of counter will be returned from the loop as a result
        }

        counter += 1;
    };

    println!(
        "The first number whose sum of all previous numbers is greater than or equal to {} is {}.",
        target, result
    );
    return result;
}

fn create_loop_and_break_label() {
    let x = 1;

    let z = 'outer: loop {
        let mut y = 1;

        // labels 操作
        'inner: loop {
            if y == 3 {
                y += 1;
                continue 'inner; // Skips to the next iteration of the 'inner loop
            }

            println!("x: {}, y: {}", x, y);

            y += 1;

            if y > 5 {
                break 'outer y; // Breaks out of the 'inner loop
            }
        }
    };
    println!("z: {}", z);
}

fn create_while() {
    let condition: bool = false;
    while condition {
        // code to execute while the condition is true
    }
}

fn create_for() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < numbers.len() {
        println!("The value is: {}", numbers[index]);
        index += 1;
    }
}
