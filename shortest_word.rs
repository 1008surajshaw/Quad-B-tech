fn shortest_word(input: &str) -> Option<&str> {
    let mut shortest = None;

    for word in input.split_whitespace() {
        if let Some(shortest_word) = shortest {
            if word.len() < shortest_word.len() {
                shortest = Some(word);
            }
        } else {
            shortest = Some(word);
        }
    }

    shortest
}

fn main() {
    let sentence = "Hello my name is suraj ";
    
    if let Some(shortest) = shortest_word(sentence) {
        println!("The shortest word in the sentence is '{}'", shortest);
    } else {
        println!("The sentence is empty");
    }
}
