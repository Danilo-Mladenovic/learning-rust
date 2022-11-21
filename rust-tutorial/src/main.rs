fn main() {
    println!("{}", remove_char("abracadabra"));
}

// Codewars Kata's for leaning rust.
//
// Remove First and Last Character
pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}
