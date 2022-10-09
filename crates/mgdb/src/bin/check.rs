use std::any::type_name;

fn type_of<T>(_: &T) -> String {
    format!("{}", type_name::<T>())
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    let s = "Hello";
    let n1 = 42;
    let n2 = 42.68;
    let b  = true;
    let mut v: Vec<String> = vec![];

    print_type_of(&s);
    print_type_of(&n1);
    print_type_of(&n2);
    print_type_of(&b);
    print_type_of(&v);

    let t1 = type_of(&s);
    let t2 = type_of(&v);
    println!("{}", t1);

    v.push(t1);
    v.push(t2);

    println!("{:#?}", v);
}