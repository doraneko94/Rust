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
    let nab = readn::<i32>(" ");
    let n = nab[0];
    let a = nab[1];
    let b = nab[2];

    let mut ans = 0;

    for i in 1..n+1 {
        let mut num = i;
        let mut sum = 0;
        loop {
            if num == 0 {
                break;
            }
            sum += num % 10;
            num /= 10;
        }
        if sum >= a && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
