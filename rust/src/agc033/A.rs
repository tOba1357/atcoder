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

fn check(H: &usize, W: &usize, i: &i32, j: &i32) -> bool {
    *H as i32 > *i && *W as i32 > *j && 0 <= *i && 0 <= *j
}

pub fn main() {
    let HW = read_vec();
    let H: usize = HW[0];
    let W: usize = HW[1];
    let mut Ass = Vec::with_capacity(H);
    for i in 0..H {
        let row: String = read();
        Ass.push(row.chars().collect::<Vec<char>>());
    }
    let mut c = 0;
    let mut poss: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..H {
        for j in 0..W {
            let c = Ass[i][j];
            if c == '#' { poss.insert((i, j)); }
        }
    }
    loop {
        if poss.len() == 0 { break; }
        c += 1;
        println!();
        poss = poss.iter().flat_map(|&(i, j)| {
            let i = i as i32;
            let j = j as i32;
            let a = i + 1;
            let b = j;
            let mut v = Vec::new();
            if check(&H, &W, &a, &b) {
                if Ass[a as usize][b as usize] == '.' {
                    v.push((a as usize, b as usize));
                }
                Ass[a as usize][b as usize]  = '#';
            }
            let a = i - 1;
            let b = j;
            if check(&H, &W, &a, &b) {
                if Ass[a as usize][b as usize] == '.' {
                    v.push((a as usize, b as usize));
                }
                Ass[a as usize][b as usize]  = '#';
            }
            let a = i;
            let b = j + 1;
            if check(&H, &W, &a, &b) {
                if Ass[a as usize][b as usize] == '.' {
                    v.push((a as usize, b as usize));
                }
                Ass[a as usize][b as usize]  = '#';
            }
            let a = i;
            let b = j - 1;
            if check(&H, &W, &a, &b) {
                if Ass[a as usize][b as usize] == '.' {
                    v.push((a as usize, b as usize));
                }
                Ass[a as usize][b as usize]  = '#';
            }
            v
        }).collect();
    }
    println!("{}", c - 1);
}