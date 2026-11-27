
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

fn solve(sc: &mut Scanner<'_>, out: &mut impl Write) {
    let (n, k, p): (usize, usize, u64) = (sc.next(), sc.next(), sc.next());
    let mut arr: Vec<u64> = sc.vec(n);

    arr.sort_unstable();
    let mut result: u64 = 0;

    for chunk in arr.chunks(k) {
        let sum: u64 = chunk.iter().sum();
        
        result += if chunk.len() == k {
            sum.max(p)
        } else {
            sum
        }
    }

    writeln!(out, "{result}");
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
