
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

enum Temperture {
    Celsius(i32),
    Fahrenheit(i32),
}

fn age() -> u32 {
    15
}

fn main() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // 匹配单个值
        1 => println!("One!"),
        // 匹配多个值
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // 匹配一个闭区间范围
        13..=19 => println!("A teen"),
        // 处理其他情况
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        // match分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);


    // match还可以对结构进行解析匹配
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (1, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    // array and slice
    let array = [0, 1, 2];
    match array {
        [0, second, third] => println!("array[0]=0, array[1]={}, array[2]={}", second, third),
        // 第二个元素
        [1, _, third] => println!("array[0]=1, array[2]={}", third),

        [-1, second, ..] => println!("array[0]=-1, array[1]={} and all the other ones were ignored", second),

        [3, second, tail@..] => {
            println!("array[0]=3, array[1]={}", second);
            println!("The rest of the array is {:?}", tail);
        },

        // 其他情况, 首尾两个元素我们单独取出
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
    }

    // 指针或引用
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 为了避免 `&` 符号，你可以先把引用去掉
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    // 这里r是一个对value的引用
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    // 由于引用借用，value仍然可以使用
    println!("value is: {}", value);

    match mut_value {
        ref mut m => {
            // 通过解构引用来改变其值
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // 结构体
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }

    // guard， 增强模式匹配, 通过`if`来增加匹配的条件
    let temperature = Temperture::Celsius(35);
    match temperature {
        Temperture::Celsius(val) if val >30 => println!("{}C is above 30 Celsius", val),
        Temperture::Celsius(val) => println!("{}C is equal to or below 30 Celsius", val),
        Temperture::Fahrenheit(val) if val > 86 => println!("{}F is above 86 Fahrenheit", val),
        Temperture::Fahrenheit(val) => println!("{}F is equal to or below 86 Fahrenheit", val),
    }

    let number: u8 = 4;
    // match会穷尽所有选项，但是不会吧guard包含在内，如果没有显示的表达
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        i  => println!("Less than zero"),
    }

    // 绑定
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}