
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
    let _: usize = sc.next();
    let mut s = sc.next::<String>().into_bytes();

    let mut pending = 0;
    let mut missing = 0;

    // Forward: identify commands that consume scanned documents.
    for command in &mut s {
        match *command {
            b'1' => {
                pending += 1;
                missing += 1;
            }
            b'2' if pending > 0 => {
                pending -= 1;
                *command = b'X';
            }
            _ => {}
        }
    }

    // Backward: match consumed documents in LIFO order.
    let mut consumed = 0;

    for command in s.iter_mut().rev() {
        match *command {
            b'X' => consumed += 1,
            b'1' if consumed > 0 => {
                consumed -= 1;
                *command = b'3';
            }
            _ => {}
        }
    }

    writeln!(out, "{missing}").unwrap();

    for (i, &command) in s.iter().enumerate() {
        if matches!(command, b'1' | b'X') {
            write!(out, "{} ", i + 1).unwrap();
        }
    }

    writeln!(out).unwrap();
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
}
