fn main() {
    for a in 1..=999 {
        for b in a+1..=999 {
            let c = 1000 - a - b;
            if c > 0 && a*a + b*b == c*c {
                println!("{}", a*b*c);
                return;
            }
        }
    }
}