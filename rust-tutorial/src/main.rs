fn main() {
    println!("{}", abbrev_name("Danilo Mladenovic"));
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
