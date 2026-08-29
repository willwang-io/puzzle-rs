// Created: Aug 27 2026, 17:18:55
// Formatted with rustfmt.

fn solve() {
    let _: usize = read();
    let s: String = read();
    let a = s.as_bytes();
    let ans = a.iter().map(|&x| (x - b'0') as i32).sum::<i32>()
        + a[..a.len() - 1].iter().filter(|&&x| x != b'0').count() as i32;
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
