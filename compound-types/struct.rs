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

struct Person {
    name: String,
    age: u8,
    hobby: String
}

fn exercise_1() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: "coding".to_string()
    };
}


struct Unit;
trait SomeTrait {
    // ...Some behavours defines here
}

// We don't care the the fields are in Unit, but we care its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit {  }

fn exercise_2() {
    let u = Unit;
    do_something_with_unit(u);
}

// fill the blank to make the code work
fn do_something_with_unit(u: Unit) {   }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn exercise_3() {
    let v = Point(0, 127, 255);
    check_color(v);
}
fn check_color(p: Point) {
    let Point(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

struct Person2 {
    name: String,
    age: u8,
}
fn exercise_4() {
    let age = 18;
    let mut p = Person2 {
        name: String::from("sunface"),
        age,
    };

    // how can you believe sunface is only 18? 
    p.age = 30;

    p.name = String::from("sunfei");
}

struct Person3 {
    name: String,
    age: u8,
}
fn exercise_5() {}
fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn exercise_6() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);
}
fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        ..u
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn exercise_7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // print debug info to stderr and assign the value of  `30 * scale` to `width`
        height: 50,
    };

    dbg!(&rect1); // print debug info to stderr

    println!("{:?}", rect1); // print debug info to stdout
}

#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn exercise_8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string()
    };

    let _name = f.name;

    println!("{}", f.data);
}