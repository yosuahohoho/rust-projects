use std::io;

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    println!("Enter the width: ");
    let width_input = user_input();

    println!("Enter the height: ");
    let height_input = user_input();

    let rect = Rectangle {
        width: width_input,
        height: height_input,
    };
    
    println!("Width: {}", rect.width);
    println!("Height: {}", rect.height);
    println!("The area of rectangle is: {}", rect.area());
}

fn user_input() -> i32 {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed read line.");

    let input: i32 = input.trim().parse()
        .expect("Please enter an integer.");

    input
}
