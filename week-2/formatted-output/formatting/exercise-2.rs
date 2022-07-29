fn main() {
    println!("{argument}", argument = "test"); // => "test"

    //formatting the text so the test passes
    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}",a = "a", b = 'b', c = 3 ), "a 3 b");
    
    println!("{abc} {0}", 2, abc = "def");

    println!("Success!")
}