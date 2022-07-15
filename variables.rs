fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
    exercise_4();
    exercise_5();
    exercise_6();
    exercise_7();
    exercise_8();
}

fn exercise_1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

fn exercise_2 () {
    let mut x =  1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

fn exercise_3 () {
    let x: i32 = 10;
    
    //moving the definition of y, outside the scope so it can be used inside and outside the scope
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

fn exercise_4 () {
    let x: String = define_x();
    println!("{}, world", x); 
}
fn define_x() -> String {
    let x = String::from("hello");
    x
}



fn exercise_5 () {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x =  42;
    println!("{}", x); // Prints "42".
}


fn exercise_6 () {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    
    //removing this line (making it comment)
    //let x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}


fn exercise_7 () {
    let x = 1; 
    
    //using the variable 
    println!("{}", x);
}


fn exercise_8 () {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
