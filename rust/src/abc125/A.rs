use std::io;

pub fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).ok();
    let line: Vec<i32> = buf.split_whitespace().into_iter().map(|s| s.parse().unwrap()).collect();
    let A = line[0];
    let B = line[1];
    let T = line[2];
    println!("{}", T / A * B);
}