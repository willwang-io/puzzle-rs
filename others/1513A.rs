// Created: Aug 30 2026, 15:36:25
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let k: usize = read();
    if k > (n - 1) / 2 {
        println!("-1");
        return;
    }
    let mut a: Vec<usize> = (1..=n).collect();
    for i in 0..k {
        a.swap(2 * i + 1, 2 * i + 2);
    }
    let ans = a
        .iter()
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
