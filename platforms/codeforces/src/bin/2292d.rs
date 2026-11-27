
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

fn solve(sc: &mut Scanner<'_>, out: &mut impl Write) {
    let n: usize = sc.next();
    let left: Vec<i128> = sc.vec(n);
    let right: Vec<i128> = sc.vec(n);

    use std::iter::zip;
    use std::cmp::Ordering;

    let mut dp: i128 = 0;

    for (x, y) in zip(&left, &right).rev() {
        let diff = x - y;
        dp = (diff + dp).max(-dp);
    }

    let result = match dp.cmp(&0) {
        Ordering::Less => "Aubrey",
        Ordering::Greater => "Sunny",
        Ordering::Equal => "Draw",
    };

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
