fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new(); 
    }

    let first_str = &strs[0]; 
    for (i, c) in first_str.chars().enumerate() {
        for s in &strs[1..] {
            if let Some(ch) = s.chars().nth(i) {
                if ch != c {
                    return first_str[..i].to_string(); 
                }
            } else {
                return first_str[..i].to_string(); 
            }
        }
    }

    first_str.to_string() 
}

fn main() {
    let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    println!("Longest common prefix: {}", longest_common_prefix(&strs)); 
}
