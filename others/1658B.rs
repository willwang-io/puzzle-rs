// Created: Aug 28 2026, 19:36:49
// Formatted with rustfmt.

fn solve() {
    let n: i64 = read();
    if n % 2 == 1 {
        println!("0");
        return;
    }
    let mut ans = 1;
    for x in 2..=n / 2 {
        ans = ans * x % 998_244_353;
    }
    ans = ans * ans % 998_244_353;
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
