// Created: Aug 30 2026, 16:13:02
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let b: String = read();
    let mut prev = -1;
    let mut ans = String::new();
    for c in b.bytes() {
        let x = (c - b'0') as i32;
        if x + 1 != prev {
            ans.push('1');
            prev = x + 1;
        } else {
            ans.push('0');
            prev = x;
        }
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
