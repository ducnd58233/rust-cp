
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
    let n: usize = sc.next();
    let mut graph = vec![Vec::new(); n];

    for _ in 0..n-1 {
        let u = sc.next::<usize>() - 1;
        let v = sc.next::<usize>() - 1;

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut parent = vec![0; n];
    let mut order = Vec::with_capacity(n);

    order.push(0);
    let mut i = 0;

    while i < order.len() {
        let u = order[i];

        for &v in &graph[u] {
            if v == parent[u] {
                continue
            }

            parent[v] = u;
            order.push(v);
        }

        i += 1;
    }

    let mut size = vec![1; n];
    let mut worst_loss = 0;

    for &u in order.iter().rev() {
        let (mut first, mut second, mut third) = (0, 0, 0);

        for &v in &graph[u] {
            if v == parent[u] {
                continue;
            }

            size[u] += size[v];
            let s = size[v];

            if s > first {
                third = second;
                second = first;
                first = s;
            } else if s > second {
                third = second;
                second = s;
            } else if s > third {
                third = s;
            }
        }

        let loss = if u == 0 { third } else { second };
        worst_loss = worst_loss.max(loss);
    }

    writeln!(out, "{}", n - worst_loss).unwrap();
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
