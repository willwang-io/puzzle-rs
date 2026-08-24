// Created: Aug 24 2026, 17:07:42
// Formatted with rustfmt.

fn main() {
    let d: f64 = read();
    let l: f64 = read();
    let v1: f64 = read();
    let v2: f64 = read();
    let ans = (l - d) / (v1 + v2);
    println!("{ans:.10}");
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
