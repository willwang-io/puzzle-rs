// Created: Sep  3 2026, 20:39:02
// Formatted with rustfmt.

fn solve() {
    let l1: i32 = read();
    let r1: i32 = read();
    let l2: i32 = read();
    let r2: i32 = read();

    let mut lst = [(l1, r1), (l2, r2)];
    lst.sort_unstable();

    if lst[1].0 > lst[0].1 {
        println!("{}", lst[0].0 + lst[1].0);
    } else {
        println!("{}", lst[0].0.max(lst[1].0));
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
