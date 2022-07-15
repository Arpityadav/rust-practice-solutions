use std::ops::{Range, RangeInclusive};

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
    exercise_8_2();
    exercise_9();
    exercise_10();
    exercise_11();
}


fn exercise_1() {
    let x: i32 = 5;
    let mut y = 5;

    y = x;
    
    let z = 10; // type of z : i32
}
fn exercise_2() {
    let v: u16 = 38_u8 as u16;
}
fn exercise_3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}

// get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn exercise_4() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255);
}
fn exercise_5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
}
fn exercise_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}
fn exercise_7() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64
}
fn exercise_8() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
}
fn exercise_8_2() {
    assert!((0.1_f64+ 0.2 - 0.3).abs() < 0.001);
}
fn exercise_9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
fn exercise_10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}
fn exercise_11() {
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    
    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
