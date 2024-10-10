use std::str::FromStr;

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    //let/else可以用于错误处理。当然也可以用if/else或match实现，但是这样更简洁
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse count as u64: '{count_str}'");
    };
    (count, item)
}

fn get_count_item_v2(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse integer: '{count_str}'");
    };

    (count, item)
}

fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // b不跟Foo::Bar匹配，所以这个分支不会执行
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }


    assert_eq!(get_count_item("5 apples"), (5, "apples"));
    assert_eq!(get_count_item_v2("5 apples"), (5, "apples"));
}