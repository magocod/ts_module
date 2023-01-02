pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false; // Corner case, early return
    }
    lhs % rhs == 0 // The last expression is the return value
}

// fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
//     match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
//         (true,  true)  => println!("fizzbuzz"),
//         (true,  false) => println!("fizz"),
//         (false, true)  => println!("buzz"),
//         (false, false) => println!("{n}"),
//     }
// }

fn fizzbuzz(n: u32, v: &mut Vec<String>) {
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => v.push("fizzbuzz".to_string()),
        (true, false) => v.push("fizz".to_string()),
        (false, true) => v.push("buzz".to_string()),
        (false, false) => v.push(n.to_string()),
    }
}

// pub fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// }

pub fn fizzbuzz_to(n: u32) -> Vec<String> {
    let mut v: Vec<String> = vec![];

    for n in 1..=n {
        fizzbuzz(n, &mut v);
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }

    #[test]
    fn check_fizzbuzz_to_5() {
        let v = fizzbuzz_to(5);
        // println!("vec: {:?}", v);
        assert_eq!(v, vec!["1", "2", "fizz", "4", "buzz"]);
    }

    #[test]
    fn check_fizzbuzz_to_20() {
        let v = fizzbuzz_to(20);
        // println!("vec: {:?}", v);
        assert_eq!(
            v,
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16", "17", "fizz", "19", "buzz"
            ]
        );
    }
}
