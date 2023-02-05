use std::mem::replace;

pub fn match_fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => match_fibonacci(n - 1) + match_fibonacci(n - 2),
    }
}

pub fn let_fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

#[inline]
pub fn inline_let_fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

// recursive fibonacci
pub fn recursive_fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

// iterative fibonacci
pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

pub fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

#[inline]
pub fn inline_fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[inline]
pub fn inline_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
