fn main() {
    exercise_1();
    exercise_1_2();
    exercise_2();
    exercise_3();
}

fn exercise_1() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());
}

fn exercise_1_2() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);
}

fn exercise_2() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
}

fn exercise_3() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}