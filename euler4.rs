fn is_palindromic(i: i32) -> bool {
    i.to_string().chars().rev().collect::<String>() == i.to_string()
}

fn main() {
    let mut res = 0;
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let prod = i * j;

            if prod > res && is_palindromic(prod) {
                res = prod;
                break;
            }
        }
    }
    println!("{}", res);
}