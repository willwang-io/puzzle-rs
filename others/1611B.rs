// Created: Aug 23 2026, 12:00:07
// Formatted with rustfmt.

fn solve() {
    let mut a: i64 = read();
    let mut b: i64 = read();
    if a > b {
        (a, b) = (b, a);
    }

    if a == b {
        println!("{}", a / 2);
    } else if a * 3 <= b {
        println!("{a}");
    } else {
        println!("{}", (a + b) / 4);
    }
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
