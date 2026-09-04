// Created: Aug 30 2026, 15:44:57
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<(i64, i64)> = (0..n).map(|_| (read(), read())).collect();
    let tm: Vec<i64> = (0..n).map(|_| read()).collect();
    let mut time = 0;
    let mut prev = 0;
    let mut ans = 0;
    for i in 0..n {
        time += a[i].0 - prev + tm[i];
        ans = time;
        time = a[i].1.max(time + (a[i].1 - a[i].0 + 1) / 2);
        prev = a[i].1;
    }
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
