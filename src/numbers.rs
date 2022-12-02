use rand::prelude::*;

/// Generates random numbers with length
pub fn get_rand(limit: i32) -> i32 {
    let mut rng = thread_rng();
    let mut nums: Vec<i32> = (1..=9).collect();
    nums.shuffle(&mut rng);

    let numbers = iter_numbers(nums, limit, &mut String::default());
    let result = numbers.parse::<i32>();
    if result.is_err() {
        return 0;
    }

    result.unwrap()
}

/// Iterates to list of numbers
pub fn iter_numbers(numbers: Vec<i32>, limit: i32, current: &mut String) -> String {
    // Return current if less than 1
    if limit < 1 {
        return current.clone();
    }

    // Create edge
    let edge = match limit > 9 {
        true => 9,
        false => limit
    };

    // Create extra
    let extra = match limit > 9 {
        true => limit - 9,
        false => 0
    };

    let nums: String = numbers[0..=( (edge - 1) as usize)].iter().map( |&id| id.to_string()).collect();
    let mut current = format!("{}{}", current, nums);


    iter_numbers(numbers, extra, &mut current)
}