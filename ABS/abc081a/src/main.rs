fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s1s2s3 = read1::<String>();
    let mut ans = 0;
    for i in 0..3 {
        let s: i32 = s1s2s3[i..i+1].parse().ok().unwrap();
        ans += s;
    }
    println!("{}", ans);
}
