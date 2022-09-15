// Problem id 2
// https://projecteuler.net/problem=2

// This calculates the sum of all even fibonacci numbers
pub fn solution(n: u32) -> u32 {
    // Initializing varialbes
    let mut result = 0;
    let mut a = 1;
    let mut b = 2;
    let mut _c = 0; // Named it with _ prefix, because the compiler was warning about it
    
    while a <= n {
        if a % 2 == 0 {
            result += a; // Adding the current even fib to the result
        }
        _c = a + b; // Calculating future fib from older fibs

        // Updating old fibs for next iteration
        a = b;
        b = _c;
    }

    result
}

#[test]
fn even_fibonacci_numbers() {
    assert_eq!(solution(34), 44);
    assert_eq!(solution(4000000), 4613732)
}
