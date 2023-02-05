use std::mem::replace;

// recursive fibonacci
pub fn fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
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

pub fn match_fibonacci(n: usize) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        n => match_fibonacci(n - 1) + match_fibonacci(n - 2),
    }
}

#[inline]
pub fn inline_fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        inline_fibonacci(n - 1) + inline_fibonacci(n - 2)
    }
}
