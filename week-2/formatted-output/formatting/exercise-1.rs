fn main() {
    //formatting the text so the test passes
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");// => Alice, this is Bob. Bob, this is Alice
    
    //formatting the text so the test passes
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    
    //formatting the text so the test passes
    assert_eq!(format!("{1}{}{0}{}", 1, 2), "2112");
    println!("Success!")
}