fn param_test(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
    let v = f(value);
    // println!("fn {}, -> param fn {}", value, v);
    v
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn main() {
    param_test(5, &times2);
}
