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
    let mut v: Vec<Vec<i32>> = vec!(vec!(0, 0, 0));
    let n = read1::<u32>();
    for _ in 0..n {
        v.push(readn::<i32>(" "));
    }

    let mut flag = true;
    for i in 0..n {
        let dt = (v[i as usize][0] - v[(i+1) as usize][0]).abs();
        let dx = (v[i as usize][1] - v[(i+1) as usize][1]).abs();
        let dy = (v[i as usize][2] - v[(i+1) as usize][2]).abs();
        if dx + dy > dt || (dt - (dx + dy)) % 2 != 0 {
            flag = false;
            break;
        }
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
