fn main() {
    exercise_1();
    exercise_1_2();
    exercise_1_3();
    exercise_1_4();
    exercise_2();
    exercise_3();
    exercise_3_2();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
}

fn exercise_1() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

fn exercise_1_2() {
    let x = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}
fn exercise_1_3() {
    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}
fn exercise_1_4() {
    let x = 10;
    let y = x;
    println!("{},{}",x,y);
}

fn exercise_2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn exercise_3() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.as_bytes();
    s
}

fn exercise_3_2() {
    let s = give_ownership_2();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership_2() -> String {
    let s = String::from("hello, world");
    s
}

fn exercise_4() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}
fn print_str(s: String)  {
    println!("{}",s)
}

fn exercise_5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
fn exercise_6() {
    let s = String::from("hello, ");
    
    // modify this line only !
    let mut s1 = s;

    s1.push_str("world")

}
fn exercise_7() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);       // implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);
}
fn exercise_8() {
    let t = (String::from("hello"), String::from("world"));
 
    let _s = t.0;
 
    // modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

fn exercise_9() {
    let t = (String::from("hello"), String::from("world"));

    // fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}