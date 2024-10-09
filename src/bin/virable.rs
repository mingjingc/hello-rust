
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variables; these warnings can be silenced
    let _unused_variable = 3u32;
    // let noisy_unused_variable = 2u32;

    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // This binding shadows the outer one
        let long_lived_binding = 5_f32;
    }

    // println!("outer long: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    // declare first
    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // 不可以使用未未初始化的变量 another_binding
    // println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);

    let mut mutable_integer = 7i32;
    {
        // Shadowing by re-declaring the variable
        let mutable_integer = mutable_integer;
        // This binding shadows the outer one
        //mutable_integer = 50;
    }
    println!("mutable integer: {}", mutable_integer);
}