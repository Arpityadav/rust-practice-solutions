fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
}

fn exercise_1() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}


fn exercise_2() {
    print();
}
fn print() -> () {
    println!("hello,world");
}

fn exercise_3() {
    never_return();
}
fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    panic!("Never return anything")
}

fn exercise_4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function
// DON'T change any code else
fn never_return_fn() -> ! {
    panic!()
}
fn exercise_5() {
    // FILL in the blank
    let b = false;

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}

