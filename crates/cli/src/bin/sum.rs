use std::ops::Add;

struct Millimeters(u32);
struct Centimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Centimeters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Centimeters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 10))
    }
}

fn main() {
    let a = Millimeters(100);
    let b = Meters(1);
    let c = Centimeters(1);

    println!("Millimeters {}", a.0);
    println!("Meters {}", b.0);
    println!("Centimeters {}", c.0);

    let a = a + b;
    let a = a + c;

    println!("sum {}", a.0);
}