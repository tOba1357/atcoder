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

enum Turn {
    S,
    T,
}

struct Simulator {
    H: usize,
    W: usize,
    N: usize,
    sr: usize,
    sc: usize,
    S: Vec<char>,
    T: Vec<char>,
}
impl Simulator {
    pub fn exec(&mut self) -> bool {
        let pos = (self.sc, self.sr).clone();
        self.r(Turn::S, &0, &pos)
    }
    fn r(&mut self, turn: Turn, i: &usize, pos: usize) -> bool {
        if pos.0 == 0 || pos.0 == (self.W + 1) || pos.1 == 0 || pos.1 == (self.H + 1) { return true }
        match turn {
            Turn::S => if *i == self.N { return false },
            Turn::T => (),
        }
        let d = match turn {
            Turn::S => self.S[*i],
            Turn::T => self.T[*i],
        };
        let next_pos = match d {
            'L' => (pos.0 - 1, pos.1),
            'R' => (pos.0 + 1, pos.1),
            'U' => (pos.0, pos.1 - 1),
            'D' => (pos.0, pos.1 + 1),
            _ => panic!("")
        };
        let result = match turn {
            Turn::S => {
                self.r(Turn::T, i, pos) || self.r(Turn::T, i, &next_pos)
            },
            Turn::T => {
                self.r(Turn::S, &(*i + 1), pos) && self.r(Turn::S, &(*i + 1), &next_pos)
            },
        };
        result
    }
}


pub fn main() {
    let HWN = read_vec();
    let H:usize = HWN[0]; let W:usize = HWN[1]; let N = HWN[2];
    let S = read_vec();
    let sr: usize = S[0]; let sc = S[1];
    let S: Vec<char> = read::<String>().chars().collect();
    let T: Vec<char> = read::<String>().chars().collect();
    let mut rs: HashSet<usize> = HashSet::with_capacity(H);
    let mut cs: HashSet<usize> = HashSet::with_capacity(W);
    let mut simulator = Simulator {
        H: H,
        W: W,
        N: N,
        sr: sr,
        sc: sc,
        S: S,
        T: T,
    };
    if !simulator.exec() {
        println!("YES");
    } else {
        println!("NO");
    }
}