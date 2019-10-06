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
    let ny = readn::<i32>(" ");
    let n = ny[0];
    let y = ny[1];

    let yukichi_max = y/10000;
    if yukichi_max > n {
        println!("-1 -1 -1");
    } else {
        let mut flag = true;
        'solve:
        for i in 0..yukichi_max+1 {
            let ichiyou_max = (y - 10000 * i) / 5000;
            if ichiyou_max > n - i {
                continue;
            } else {
                for j in 0..ichiyou_max+1 {
                    if 10000 * i + 5000 * j + 1000 * (n - i - j) == y {
                        println!("{} {} {}", i, j, n - i - j);
                        flag = false;
                        break 'solve
                    }
                }
            }
        }
        if flag {
            println!("-1 -1 -1");
        }
    }
}
