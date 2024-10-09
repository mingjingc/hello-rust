#![allow(overflowing_literals)]

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    println!("{:b}",1000);

    println!("1000 as a u8 is : {}", 1000 as u8);
    // float不能直接转换成char
    // let character = decimal as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);


    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // 可以推导类型
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // 编译器不会把别名当作一个新类型
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}