fn main() {
    let mut i = 19;
    let mut n = 20;
    while i >= 2 {
        if n % i != 0 {
            n += 20;
            i = 20;
        }
        i -= 1;
    }
    println!("{}", n);
}