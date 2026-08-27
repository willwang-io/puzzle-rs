// Created: Aug 25 2026, 11:23:08
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut height = 1;
    let mut prev = -1;
    let mut dead = false;

    for _ in 0..n {
        let x: i32 = read();
        if x == 0 && prev == 0 {
            dead = true;
        } else if x == 1 {
            height += if prev == 1 { 5 } else { 1 };
        }
        prev = x
    }

    let ans = if dead { -1 } else { height };
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
