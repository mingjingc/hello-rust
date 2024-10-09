use std::arch::is_aarch64_feature_detected;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);
impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

struct Circle {
    radius: i32
}
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    // let num = Number::from(30);
    // println!("My number is {:?}", num);

   //  let int = 5;
   //  // 实现了from，Into可以调用。但是如果只是实现了into，from不能调用
   //  let num: Number = int.into();
   //  println!("My number is {:?}", num);
   //
   //  // TryFrom
   //  assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
   // // assert_eq!(EvenNumber::try_from(5), Err(()));
   //
   //  // TryInto
   //  let result: Result<EvenNumber, ()> = 8i32.try_into();
   //  assert_eq!(result, Ok(EvenNumber(8)));
   //  let result: Result<EvenNumber, ()> = 5i32.try_into();
   //  assert_eq!(result, Err(()));

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}