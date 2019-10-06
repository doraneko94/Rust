fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>();
    let length = s.len();
    let mut pos = 0;
    let mut flag = 0;

    while pos < length {
        if pos+7 <= length && s[pos..pos+7] == "dreamer".to_string() {
            pos += 7;
            flag = 2;
        } else if pos+6 <= length && s[pos..pos+6] == "eraser".to_string() {
            pos += 6;
            flag = 1;
        } else if pos+5 <= length && s[pos..pos+5] == "dream".to_string() {
            pos += 5;
            flag = 0;
        } else if pos+5 <= length && s[pos..pos+5] == "erase".to_string() {
            pos += 5;
            flag = 0;
        } else if flag > 0 {
            pos -= flag;
            flag = 0;
        } else {
            break;
        }
    }
    if pos == length {
        println!("YES");
    } else {
        println!("NO");
    }
}
