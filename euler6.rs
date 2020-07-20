fn main() {
    let mut s = 0;
    let mut sq = 0;
    for i in 1..=100 {
        s += i;
        sq += i * i;
    }
    println!("{}", s * s - sq);
}