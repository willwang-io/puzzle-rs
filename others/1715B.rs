// Created: Aug 24 2026, 17:17:13
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: i64 = read();
    let b: i64 = read();
    let s: i64 = read();
    let min = b * k;
    let max = min + (k - 1) * n as i64;

    if s < min || s > max {
        println!("-1");
        return;
    }

    let mut a = vec![0; n];
    a[0] = min;
    let mut extra = s - min;

    for x in &mut a {
        let add = extra.min(k - 1);
        *x += add;
        extra -= add;
    }

    let ans = a
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{ans}");
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
    }
}

thread_local! {
    pub static INPUT: std::cell::RefCell<std::str::SplitAsciiWhitespace<'static>> = std::cell::RefCell::<std::str::SplitAsciiWhitespace<'static>>::new({
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        Box::leak(input.into_boxed_str()).split_ascii_whitespace()
    });
}

pub fn read<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    INPUT.with(|input| input.borrow_mut().next().unwrap().parse().unwrap())
}
