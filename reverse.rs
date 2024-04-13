fn reverse_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }
    chars.into_iter().collect()
}

fn main() {
    let s = "Hello, world!";
    let reversed = reverse_string(s);
    println!("Original: {}", s);
    println!("Reversed: {}", reversed);
}
