// Problem id 6
// https://projecteuler.net/problem=6

// This calculates the the difference between
// the square of sum of natural numbers upto n
// and sum of squares of natural numbers upto n
pub fn solution(n: i32) -> i32 {
    // Sum of natural numbers 1 to n = (n * (n + 1)) / 2
    let sum_squared = ((n * (n + 1)) / 2) * ((n * (n + 1)) / 2);

    // Sum of squares of numbers 1 to n = (n * (n + 1) * (2 * n + 1)) / 6
    let sum_of_squares = (n * (n + 1) * (2 * n + 1)) / 6;

    sum_squared - sum_of_squares
}

#[test]
fn testoooo() {
    assert_eq!(solution(10), 2640);
    assert_eq!(solution(100), 25164150);
}
