fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

use std::collections::HashSet;

fn main() {
    let mut uniq = HashSet::new();

    let n = read1::<u32>();
    for _ in 0..n {
        uniq.insert(read1::<u32>());
    }
    println!("{}", uniq.len());
}
