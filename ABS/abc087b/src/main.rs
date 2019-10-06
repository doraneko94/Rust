fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let a = read1::<u32>();
    let b = read1::<u32>();
    let c = read1::<u32>();
    let x = read1::<u32>();

    let mut ans = 0;

    for a_num in 0..a+1 {
        for b_num in 0..b+1 {
            for c_num in 0..c+1 {
                if 500 * a_num + 100 * b_num + 50 * c_num == x {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
