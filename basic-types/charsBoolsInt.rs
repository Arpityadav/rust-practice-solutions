use std::mem::size_of_val;

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
}

fn exercise_1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),1); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),3); 

    println!("Success!");
}


fn exercise_2() {
    let c1 = "中";
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}

fn exercise_3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("hello, world");
    }
}
fn exercise_4() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);
}
fn exercise_5() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will returen a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will returen a ()")
}


fn exercise_6() {
    let unit: () = ();
    // unit type does't occupy any memeory space
    assert!(size_of_val(&unit) == 0);
}
