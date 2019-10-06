fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readn<T: std::str::FromStr>(delimeter: &str) -> Vec<T> {
    let s = read1::<String>();
    s.split(delimeter).map(|e| e.parse().ok().unwrap()).collect::<Vec<T>>()
}

fn main() {
    let ab = readn::<i32>(" ");
    let a = &ab[0];
    let b = &ab[1];
    if a * b % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
