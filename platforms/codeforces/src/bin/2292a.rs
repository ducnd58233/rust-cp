
use std::io::{self, Read, Write};
use std::str::{FromStr, SplitWhitespace};

struct Scanner<'a> {
    tokens: SplitWhitespace<'a>,
}

#[allow(dead_code)]
impl<'a> Scanner<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: input.split_whitespace(),
        }
    }

    fn next<T: FromStr>(&mut self) -> T {
        self.tokens.next().unwrap().parse().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn visited_mask(l: u32, step: u32) -> u32 {
    let mut mask = 0;
    let mut pos = 0;

    loop {
        let bit = 1_u32 << pos;

        if mask & bit != 0 {
            break;
        }

        mask |= bit;
        pos = (pos + step) % l;
    }

    mask
}

fn solve(sc: &mut Scanner<'_>, out: &mut impl Write) {
    let (l, a, b): (u32, u32, u32) = (sc.next(), sc.next(), sc.next());
    
    let mask_a = visited_mask(l, a);
    let mask_b = visited_mask(l, b);

    let result = (mask_a & mask_b).count_ones();

    writeln!(out, "{result}").unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut sc = Scanner::new(&input);
    let mut out = io::BufWriter::new(io::stdout().lock());

    // Multiple test cases
    let t: usize = sc.next();

    for _ in 0..t {
        solve(&mut sc, &mut out);
    }

    // solve(&mut sc, &mut out);
}
