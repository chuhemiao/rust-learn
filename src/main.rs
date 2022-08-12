use std::io;
fn main() {
    // 可变类型
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    // 常量 =》Rust 对常量的命名约定是在单词之间使用全大写加下划线。
    // 常量在整个程序生命周期中都有效
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    const PY_NUM: f64 = 3.1415926;

    let x = x + 1;
    {
        let x = x * 2;
        println!("x is new: {x} ");
    }
    // let 可以重复使用, 不能改变变量的类型

    println!("The time of const is: {THREE_HOURS_IN_SECONDS}");
    println!("The π of const is: {PY_NUM}");
    // 数据类型
    // 静态类型（statically typed）语言
    // rust 中, 每一个值都属于某一个 数据类型
    //两类数据类型子集：标量（scalar）和复合（compound）
    //标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
    // 整数 是一个没有小数部分的数字。
    // 有符号整数类型以 i 开头 无符号整数 u开头

    // Rust 也有两个原生的 浮点数（floating-point numbers）类型，它们是带小数点的数字。Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。
    // f32 是单精度浮点数，f64 是双精度浮点数。
    let fx = 2.0; // f64
    let fy: f32 = 3.0; // f32
    println!("fx is f64:{fx}");
    println!("fy is f32:{fy}");
    // 所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。
    // addition
    let sum = 5 + 10;
    println!("sum of number :{sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of number :{difference}");

    // multiplication
    let product = 4 * 30;
    println!("product of number :{product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("quotient of number :{quotient}");
    println!("floored of number :{floored}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder of number :{remainder}");

    //布尔类型有两个可能的值：true 和 false 主要用在条件句
    let t = true;

    let f: bool = false;
    println!("bool is true:{t}");
    println!("bool is false:{f}");
    // 字符串=》 用单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c char is z:{c}");
    println!("z char is Z:{z}");
    println!("face char is face:{heart_eyed_cat}");
    //复合类型（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：元组（tuple）和数组（array）
    // 元组长度声明后 固定大小 每个数据要有类型 使用模式匹配或解构获得数据
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x1, y1, z1) = tup;

    println!("The value of y1 is: {y1}");
    println!("The value of y1 is: {x1}");
    println!("The value of y1 is: {z1}");
    let five_hundred = tup.0;
    let one = tup.2;
    println!("tup 0 number: {five_hundred}");
    println!("tup 2 number: {one}");
    // 与元组不同，数组中的每个元素的类型必须相同, 数组并不如 vector 类型灵活
    // 数组是可以在堆栈上分配的已知固定大小的单个内存块。可以使用索引来访问数组的元素
    let array = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let aaaaa = [3; 5]; //变量名为 aaaaa 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。
    let array_index = array[0];
    let months_index = months[1];
    let aaaaa_index = aaaaa[2];
    println!("array is 0: {array_index}");
    println!("array is 0: {months_index}");
    println!("array is 0: {aaaaa_index}");
    // 无效的索引
    let az = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = az[index];
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 9',
    println!("The value of the element at index {index} is: {element}");
    //函数和变量名使用 snake case 规范风
    // 在 snake case 中，所有字母都是小写并使用下划线分隔单词。
    // 如果输入抱错, 不会往下执行
    // 当定义多个参数时，使用逗号分隔
    // char 4个字节,单字符
    println!("Hello, func!");

    another_function(10, "chuhemiao".to_string());

    // 条件语句
    // 并不会尝试自动地将非布尔值转换为布尔值。
    // 条件前后需要类型相同
    let q = 3;
    if q > 10 {
        println!("this right: {q}");
    }
    // if q {
    //     println!("error bool");
    // }
    if q % 2 == 0 {
        println!("number is divisible by 4");
    } else if q % 3 == 0 {
        println!("number is divisible by 3");
    } else if q % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    // 直接通过let 进行变量的简单判断
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // 有三种循环：loop、while 和 for
    // loop 无限循环,除非给出停止
    // 用break 和 continue 停止某个步骤
    // loop {
    //     println!("chuhemiao!");
    // }
    let mut count = 0;
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
    // while 条件为真就执行
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }
    // rev 反转 range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    // 斐波
    let mut fibnum = 0;
    loop {
        fibnum += 1;
        if fibnum > 10 {
            break;
        } else {
            print!("{:>2}: {:<20}", fibnum, fibonacci(fibnum));
        }
        if fibnum % 3 == 0 {
            print!("\n");
        }
    }
    // 所有权

    let mut sp = String::from("hello");
    sp.push_str(", chuhemiao!"); // push_str() 在字符串后追加字面值

    println!("{}", sp); // hello chuhemiao

    let s2 = sp.clone();

    println!("s1 = {}, s2 = {}", sp, s2);

    // 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
    // 与指针不同，引用确保指向某个特定类型的有效值。
    // 下面是如何定义并使用一个（新的）calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权：

    let len = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len);

    let mut s3 = String::from("hellokk");

    {
        let r1 = &mut s3;
        r1.push_str(", world");
        println!("{}", r1);
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    println!("s3 is {}", s3);

    let r2 = &mut s3;

    println!("r2 is {}", r2);
}

fn another_function(x: i32, y_char: String) {
    println!("Another function.");
    // 宏会把 输入的值放在格式字符串中包含 x 的那对花括号的位置
    println!("the number is and char is:  {x} {y_char}");
}

fn fibonacci(num: u32) -> u32 {
    // if num == 1 || num == 2 {
    //     1
    // } else {
    //     fibonacci(num - 1) + fibonacci(num - 2)
    // }
    match num {
        0 => 0,
        1 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
