/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

type matrix22 = ((i32, i32), (i32, i32));
fn multiply(a: matrix22, b: matrix22) -> matrix22 {
    (
        (a.0.0 * b.0.0 + a.0.1 * b.1.0, a.0.0 * b.0.1 + a.0.1 * b.1.1),
        (a.1.0 * b.0.0 + a.1.1 * b.1.0, a.1.0 * b.0.1 + a.1.1 * b.1.1)
    )
}

pub fn fib(n: i32) -> i32 {
    // DONE: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n == 0 {
        return 0;
    }
    let mut res = ((1, 0), (0, 1));
    let mut acc = ((0, 1), (1, 1));
    let mut n = n-1;
    while n > 0 {
        if n % 2 == 1 {
            res = multiply(res, acc);
        }
        acc = multiply(acc, acc);
        n >>= 1;
    }
    res.1.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
