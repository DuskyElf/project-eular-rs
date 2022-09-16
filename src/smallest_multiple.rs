// Problem id 5
// https://projecteuler.net/problem=5

// This calculates the smallest common multiple in descendingly sorted numbers
// Uses recursion
pub fn solution(mut numbers: Vec<i32>, current: i32) -> i32 {
    // Finding least common multiple between two numbers
    if numbers.len() == 1 {
        let max = numbers[0];
        let min = current;
        let mut i = 1;
        let mut trial = max * i;

        // Trying for next multiple of bigger untill it's divisible by the min number
        while trial % min != 0 {
            i += 1;
            trial = max * i;
        }

        return trial;
    }

    let number = numbers.pop().unwrap();
    // LCM (xs:x, y) = LCM (xs, LCM([x], y))
    solution(numbers, solution(vec![number], current))
}

#[test]
fn smallest_multiple() {
    assert_eq!(solution((1..10).rev().collect(), 1), 2520);
    assert_eq!(solution((1..20).rev().collect(), 1), 232792560);
}
