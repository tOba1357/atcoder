use std::collections::HashSet;

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

fn euclidean(a: i32, b: i32) -> i32 {
    if a < b {
        return euclidean(b, a);
    }
    if a % b == 0 { return b; }
    euclidean(b, a % b)
}

pub fn main() {
    let N: i32 = read();
    let As: Vec<i32> = read_vec();
    let mut befores = HashSet::new();
    let mut raw = As[0];
    befores.insert(As[0]);
    befores.insert(As[1]);
    raw = euclidean(As[0], As[1]);
    for a in As[2..As.len()].iter() {
        befores = befores.iter().map(|i| euclidean(*a, *i)).collect();
        befores.insert(raw);
        raw = euclidean(raw, *a);
    }
    println!("{}", befores.iter().max().unwrap());
}