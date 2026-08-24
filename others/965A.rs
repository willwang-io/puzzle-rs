// Created: Aug 24 2026, 16:43:04
// Formatted with rustfmt.

fn main() {
    let k: i32 = read();
    let n: i32 = read();
    let s: i32 = read();
    let p: i32 = read();
    let sheets = k * ((n + s - 1) / s);
    let ans = (sheets + p - 1) / p;
    println!("{ans}");
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
