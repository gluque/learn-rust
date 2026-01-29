#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn contains(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 23,
        height: 45,
    };

    let r2 = Rectangle {
        width: 10,
        height: 20,
    };

    let r3 = Rectangle::square(50);

    let a = r1.area();
    println!("Area of rectangle r1 is {a}");
    println!("r1 -> {r1:?}");

    println!("r2 contains r1? -> {}", r2.contains(&r1));
    println!("r1 contains r2? -> {}", r1.contains(&r2));
    println!("r1 contains r3? -> {}", r1.contains(&r3));
}
