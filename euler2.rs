fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut s = 0;
    loop {
        let c = a + b;
        if c > 4000000 { break; }
        if c % 2 == 0 {
            s += c;
        }
        a = b;
        b = c;
    }    
    println!("{}", s);
}