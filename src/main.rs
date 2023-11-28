use std::io;
use std::f32::consts::PI;

// Define a struct for a Rectangle
struct Rectangle {
    width: f32,
    height: f32,
}

// Implement methods for the Rectangle struct
impl Rectangle {
    // Constructor to create a new Rectangle
    fn new(width: f32, height: f32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
    // Method to calculate the area of the Rectangle
    fn area(&self) -> f32 {
        self.width * self.height
    }
    // Method to display information about the Rectangle
    fn show(&self) {
        println!("Area of {}x{} is {}", self.width, self.height, self.area());
    }
}
// Define a struct for a Triangle
struct Triangle {
    width: f32,
    height: f32,
}
// Implement methods for the Triangle struct
impl Triangle {
    // Constructor to create a new Triangle
    fn new(width: f32, height: f32) -> Triangle {
        Triangle {
            width,
            height,
        }
    }
    // Method to calculate the area of the Triangle
    fn area(&self) -> f32 {
        self.width * self.height * 0.5
    }
    // Method to display information about the Triangle
    fn show(&self) {
        println!("Area of Triangle {}x{} is {}", self.width, self.height, self.area());
    }
}

struct Circle {
    diameter: f32,
}
// Implement methods for the Circle struct
impl Circle {
    // Constructor to create a new Circle
    fn new(diameter: f32) -> Circle {
        Circle {
            diameter,
        }
    }
    // Method to calculate the area of the Circle
    fn area(&self) -> f32 {
        self.diameter.powf(2.0) * PI / 4.0
    }
    // Method to display information about the Circle
    fn show(&self) {
        println!("Area of Circle diameter{} is {}", self.diameter, self.area());
    }
}

// Function to get user input as a floating-point number
fn input(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please input a valid number")
}

fn main() {
    loop {
        println!("Welcome to the Area Calculator");
        println!("r: Rectangle // t: Triangle // c: Circle // q: for end");

        let mut ty = String::new();
        io::stdin().read_line(&mut ty).expect("Failed to read line");
        let ty = ty.trim();

        match ty {
            "q" => {
                println!("See you later.. bye");
                break; // Exit the loop if 'q' is entered
            }
            "r" => {
                println!("Welcome to the Rectangle calculator");
                let w = input("Write Width:");
                let h = input("Write Height");
                let r = Rectangle::new(w, h);
                r.show();
            }
            "t" => {
                println!("Welcome to the Triangle calculator");
                let w = input("Write Width:");
                let h = input("Write Height");
                let t = Triangle::new(w, h);
                t.show();
            }
            "c" => {
                println!("Welcome to the Circle calculator");
                let d = input("Write Diameter:");
                let c = Circle::new(d);
                c.show();
            }
            _ => print!("Please select others")
        }
    }
}
