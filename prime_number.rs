fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; 
    }
    let sqrt_num = (num as f64).sqrt() as u64;

    for i in 2..=sqrt_num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 15;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
