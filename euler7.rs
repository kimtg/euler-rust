fn is_prime(n: i32) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(i) {
            count += 1;
            if count >= 10001 {
                println!("{}", i);
                break;
            }
        }
        i += 1;
    }
}