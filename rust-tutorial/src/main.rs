fn main() {
    println!("{}", make_upper_case("abracadabra"));
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
