#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> String {
        match self {
        TrafficLightColor::Red => String::from("red"),
        TrafficLightColor::Yellow => String::from("yellow"),
        TrafficLightColor::Green => String::from("green")
        }
        
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}",c);
}