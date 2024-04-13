fn is_palindrome(input: &str) -> bool {
    let string_to_lowercase = input.to_lowercase();
    let reversed_string: String = string_to_lowercase.chars().rev().collect();

    string_to_lowercase == reversed_string
}

fn main() {
    let str: &str = "hello";
    println!("'{}' is{} a palindrome", str, if is_palindrome(str) { "" } else { " not" });
}
