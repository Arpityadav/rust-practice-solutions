fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_9();
    exercise_10();
    exercise_11();
}

fn exercise_1() {
    let x = 5;
    // fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // one possible output: 0x16fa3ac84
}

fn exercise_2() {
    let x = 5;
    let y = &x;

    // modify this line only
    assert_eq!(5, *y);
}

fn exercise_3() {
    let mut s = String::from("hello, ");

    borrow_object(&s)
}
fn borrow_object_1(s: &String) {}


fn exercise_4() {
    let mut s = String::from("hello, ");

    push_str(&mut s)
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

fn exercise_5() {
    let mut s = String::from("hello, ");

    // fill the blank to make it work
    let p = &mut s;
    
    p.push_str("world");
}
fn exercise_6() {
    let c = '中';

    let r1 = &c;
    // fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));
}

// get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn exercise_7() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}
fn exercise_8() {
    //fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object_2(&mut s)
}
fn borrow_object_2(s: &mut String) {}

fn exercise_9() {
    let mut s = String::from("hello, ");

    borrow_object_3(&s);
    
    s.push_str("world");
}
fn borrow_object_3(s: &String) {}

fn exercise_10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1);
}

fn exercise_11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // you can't use r1 and r2 at the same time
    r1.push_str("world");
}