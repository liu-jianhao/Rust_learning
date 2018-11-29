trait Shape {
    fn area(&self) -> u32;
}

struct Rectangle {
    x: u32,
    y: u32,
}

struct Circle {
    radius: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        self.x * self.y
    }
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        (3.14 * self.radius * self.radius) as u32
    }
}

fn main() {
    let c = Circle {
        radius: 10.36,
    };
    
    let r = Rectangle {
        x: 10,
        y: 20,
    };

    println!("{} {}", c.area(), r.area());
}
