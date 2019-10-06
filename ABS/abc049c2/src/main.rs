fn read1<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s = read1::<String>();
    let length = s.len();
    let mut pos = length;

    while pos > 0 {
        if pos >= 7 && s[pos-7..pos] == "dreamer".to_string() {
            pos -= 7;
        } else if pos >= 6 && s[pos-6..pos] == "eraser".to_string() {
            pos -= 6;
        } else if pos >= 5 && s[pos-5..pos] == "dream".to_string() {
            pos -= 5;
        } else if pos >= 5 && s[pos-5..pos] == "erase".to_string() {
            pos -= 5;
        } else {
            break;
        }
    }
    if pos == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}