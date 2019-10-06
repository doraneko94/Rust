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
    let n = read1::<i32>();
    let mut a = readn::<i32>(" ");
    let mut alice = 0;
    let mut bob = 0;

    if n % 2 == 1 {
        a.push(0);
    }
    a.sort();
    for i in 0..(n+1)/2 {
        bob += a[(2 * i) as usize];
        alice += a[(2 * i + 1) as usize];
    }
    
    println!("{}", alice - bob);
}
