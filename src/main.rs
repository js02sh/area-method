use std::io;

struct Rectangle {
    width: f32, 
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Rectangle{
        Rectangle { 
            width, 
            height, 
        }
    }
    fn area(&self) -> f32 {
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

fn input(ino: &str) -> f32 {
    println!("{}",ino);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Faild to read line");
    s.trim().parse().expect("Please input valid number")
}


fn main() {
    // let o = Rectangle {
    //     width: 10,
    //     height: 12,
    // };
    // let j = Rectangle::new(11,12);
    // let k = Triangle::new(12.0,23.0);

    // println!("Area of {}x{} is {}", o.width, o.height, o.width*o.height);
    // j.show();
    // k.show();

    loop {
        println!("Welcome to the Area Calculator");
        println!("r: Rectangle // t: Triangle // q: for end");
        let mut ty = String::new();
        io::stdin().read_line(&mut ty)
            .expect("Faild to read line");
        let ty = ty.trim();
        match ty {
            "q" => {
                println!("See you later..");
                break; // Exit the loop if 'q' is entered
            }
            "r" => {
                println!("Welclome to the Rectangle calculator");
                let w = input("Write Wtidth:");
                // println!("Write Width:");
                // let mut w = String::new();
                // io::stdin().read_line(&mut w).expect("Faild to read line");
                // let w: u32 = w.trim().parse().expect("input num pls");
                // println!("Write Height:");
                // let mut h = String::new();
                let h = input("Write Height");
                // io::stdin().read_line(&mut h).expect("Faild to read line");
                // let h: u32 = h.trim().parse().expect("input num pls");

                let r = Rectangle::new(w,h);
                r.show();
            }
            "t" => {
                println!("Welclome to the Triangle calculator");
                let w = input("Write Wtidth:");
                let h = input("Write Height");
                // println!("Write Width:");
                // let mut w = String::new();
                // io::stdin().read_line(&mut w).expect("Faild to read line");
                // let w: f32 = w.trim().parse().expect("input num pls");
                // println!("Write Height:");
                // let mut h = String::new();
                // io::stdin().read_line(&mut h).expect("Faild to read line");
                // let h: f32 = h.trim().parse().expect("input num pls");

                let r = Triangle::new(w,h);
                r.show();
            }
            _ => print!("bye")
        }

        
    }
}