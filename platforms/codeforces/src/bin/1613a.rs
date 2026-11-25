
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
    use std::cmp::Ordering;

    let (x1, p1): (u64, u32) = (sc.next(), sc.next());
    let (x2, p2): (u64, u32) = (sc.next(), sc.next());

    let common = p1.min(p2);

    let a = x1 * 10_u64.pow((p1 - common).min(7));
    let b = x2 * 10_u64.pow((p2 - common).min(7));

    let result = match a.cmp(&b) {
        Ordering::Less => '<',
        Ordering::Greater => '>',
        Ordering::Equal => '=',
    };

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
