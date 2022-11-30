use std::collections::HashMap;
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
    let digits = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let mut digit_to_char: HashMap<i32, &str> = HashMap::new();
    digit_to_char.insert(1000, "M");
    digit_to_char.insert(900, "CM");
    digit_to_char.insert(500, "D");
    digit_to_char.insert(400, "CD");
    digit_to_char.insert(100, "C");
    digit_to_char.insert(90, "XC");
    digit_to_char.insert(50, "L");
    digit_to_char.insert(40, "XL");
    digit_to_char.insert(10, "X");
    digit_to_char.insert(9, "IX");
    digit_to_char.insert(5, "V");
    digit_to_char.insert(4, "IV");
    digit_to_char.insert(1, "I");
    let mut what: &str;

    while number != 0 {
        for digit in digits {
            if number >= digit {
                what = digit_to_char.get(&digit).unwrap();
                roman_number.push_str(&what.to_string());
                number -= digit;
                break;
            }
        }
    }

    roman_number
}

/*
fn num_as_roman(mut num: i32) -> String {
    let mut roman_number = String::from("");
    let symbols = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for &(n, symbol) in symbols.iter() {
        while num >= n {
            roman_number.push_str(symbol);
            num -= n;
        }
    }

    roman_number
}
*/
