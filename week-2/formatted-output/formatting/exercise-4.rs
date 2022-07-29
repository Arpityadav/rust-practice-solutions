fn main() {

    //applying proper padding for the test to pass
    
    // left align
    println!("Hello {:<5}!", "x"); // => Hello x    !
    // right align
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    // center align
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    // left align, pad with '&'
    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!")
}