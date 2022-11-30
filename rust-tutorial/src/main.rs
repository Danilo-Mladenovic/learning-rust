use std::{fmt::format, ops::Add};

fn main() {
    println!("{}", num_as_roman(17292));
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

// Return Roman Numeral representation of given integer.
fn num_as_roman(num: i32) -> String {
    let mut roman_number = String::from("");
    let mut number = num;

    while number != 0 {
        if number >= 1000 {
            roman_number.push('M');
            number -= 1000;
        } else if number >= 900 {
            roman_number.push('C');
            roman_number.push('M');
            number -= 900;
        } else if number >= 500 {
            roman_number.push('D');
            number -= 500;
        } else if number >= 400 {
            roman_number.push('C');
            roman_number.push('D');
            number -= 400;
        } else if number >= 100 {
            roman_number.push('C');
            number -= 100;
        } else if number >= 90 {
            roman_number.push('X');
            roman_number.push('C');
            number -= 90;
        } else if number >= 50 {
            roman_number.push('L');
            number -= 50;
        } else if number >= 40 {
            roman_number.push('X');
            roman_number.push('L');
            number -= 40;
        } else if number >= 10 {
            roman_number.push('X');
            number -= 10;
        } else if number >= 9 {
            roman_number.push('I');
            roman_number.push('X');
            number -= 9;
        } else if number >= 5 {
            roman_number.push('V');
            number -= 5;
        } else if number >= 4 {
            roman_number.push('I');
            roman_number.push('V');
            number -= 4;
        } else if number >= 1 {
            roman_number.push('I');
            number -= 1;
        }
    }

    roman_number
}
