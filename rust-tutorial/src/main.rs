use std::fmt::format;

fn main() {
    println!("{}", check_for_factor(9, 2));
}

// Codewars Kata's for leaning rust.
//
// Remove first and last character.
fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

// Convert the input string to uppercase.
fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

// Convert a name into initials.
fn abbrev_name(name: &str) -> String {
    let mut splits = name.split(' ');
    let mut initials = String::new();

    initials.push_str(&splits.next().unwrap()[..1].to_uppercase());
    initials.push('.');
    initials.push_str(&splits.next().unwrap()[..1].to_uppercase());

    initials
}

// Transform a number into a string.
fn number_to_string(i: i32) -> String {
    i.to_string()
}

// Summation of numbers until n.
// n * (n + 1) / 2 is best solution.
fn summation(n: i32) -> i32 {
    let mut sum: i32 = 0;

    for i in 1..=n {
        sum += i;
    }
    sum
}

// Miliseconds past since midnight.
fn past(h: i32, m: i32, s: i32) -> i32 {
    s * 1000 + m * 60_000 + h * 3_600_000
}

// Transfer speed from km/h to cm/s
fn cockroach_speed(s: f64) -> i64 {
    (s * 27.78) as i64
}

// Return product of array members.
fn grow(nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

// Return formated string.
fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

// Test if the factor is a factor of base.
fn check_for_factor(base: i32, factor: i32) -> bool {
    base % factor == 0
}
