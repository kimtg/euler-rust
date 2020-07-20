fn main() {
    let mut num = 600851475143i64;
    let mut p = 2;
    loop {
        while num % p == 0 {
            num /= p;
        }
        if num <= 1 {
            println!("{}", p);
            break;
        }
        p += 1;
    }    
}