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
    let a = read1::<i32>();
    let bc = readn::<i32>(" ");
    let b = &bc[0];
    let c = &bc[1];
    let s = read1::<String>();
    println!("{} {}", a+b+c, s);
}
