fn main() {
    println!("{}", past(1, 1, 1));
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
