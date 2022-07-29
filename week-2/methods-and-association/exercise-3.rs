struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    // The &self is actually short for self: &Self, so using 'self: &Self' instead of '&self' in this case
    pub fn show_state(self: &Self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    //Using mut because we need to change the 'value of state'
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }
}
fn main() {
    println!("Success!");
}
