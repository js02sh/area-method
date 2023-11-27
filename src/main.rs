

struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle{
        Rectangle { 
            width, 
            height, 
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn show(&self) {
        println!("Area of {}x{} is {}", self.width, self.height, self.area());
    }
}

struct Triangle {
    width: f32,
    height: f32,
}

impl Triangle {
    fn new(width: f32, height: f32) -> Triangle{
        Triangle { 
            width, 
            height, 
        }
    }
    fn area(&self) -> f32 {
        self.width * self.height * 0.5
    }
    fn show(&self) {
        println!("Area of Triangle {}x{} is {}", self.width, self.height, self.area());
    }
}


fn main() {
    let o = Rectangle {
        width: 10,
        height: 12,
    };
    let j = Rectangle::new(11,12);
    let k = Triangle::new(12.0,23.0);

    println!("Area of {}x{} is {}", o.width, o.height, o.width*o.height);
    j.show();
    k.show();
}