use ::std::collections::HashSet;

fn read<T: ::std::str::FromStr>() -> T {
    let mut s = String::new();
    ::std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: ::std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    ::std::io::stdin().read_line(&mut s).ok();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

pub fn main() {
    let N: i64 = read();
    let As: Vec<i64> = read_vec();
    if N == 1 {
        println!("{}", As[0].abs());
        return;
    }
    let is_include_0 = As.iter().any(|&v| v == 0);
    let is_odd: i64 = As.iter().map(|&v| if v < 0 {1} else {0}).sum();
    let sum: i64 = As.iter().map(|&v| v.abs()).sum();
    if is_include_0 || is_odd % 2 == 0 {
        println!("{}", sum);
    } else {
        let sub = As.iter().map(|&v| v.abs()).min().unwrap();
        println!("{}", sum - sub * 2);
    }
}

