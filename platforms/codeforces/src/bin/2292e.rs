
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

    fn next_str(&mut self) -> &'a str {
        self.tokens.next().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn min_dist(rounds: u64, blocks: u64) -> u64 {
    let q = rounds / blocks;
    let r = rounds % blocks;

    let short = (1_u64 << q) - 1;
    let long = (1_u64 << (q + 1)) - 1;

    (blocks - r) * short + r * long
}

fn solve(sc: &mut Scanner<'_>, out: &mut impl Write) {
    let (a, x, k): (u64, u64, u64) = (sc.next(), sc.next(), sc.next());

    let blocks = k + 1;
    let (mut lo, mut hi) = (0, 30 * blocks);

    while lo + 1 < hi {
        let mi = lo + (hi - lo) / 2;

        if a + mi * x > min_dist(mi, blocks) {
            lo = mi;
        } else {
            hi = mi;
        }
    }

    writeln!(out, "{}", a + lo * x).unwrap();
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
