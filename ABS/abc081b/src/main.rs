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
    let N = read1::<u32>();
    let mut A = readn::<u32>(" ");
    let mut cnt = 0;
    'solve:
    loop {
        for i in 0..N {
            if A[i as usize] % 2 == 0 {
                A[i as usize] /= 2;
            } else {
                break 'solve;
            }
        }
        cnt += 1;
     }
    println!("{}", cnt);
}
