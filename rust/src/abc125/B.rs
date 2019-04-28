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

fn main() {
    let N: i32 = read();
    let Vs: Vec<i32> = read_vec();
    let Cs: Vec<i32> = read_vec();
    let r: i32 = Vs.iter().zip(Cs.iter())
        .map(|(v, c)| v -c)
        .filter(|v| *v > 0)
        .sum();
    println!("{}", r);
}