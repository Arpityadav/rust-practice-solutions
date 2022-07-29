fn main() {
    let s1 = "hello";
    //using format! macro
    let s = format!("{}, world!", s1);

    //asserting equals
    assert_eq!(s, "hello, world!");
}