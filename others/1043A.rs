// Created: Aug 22 2026, 22:52:42
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();

    let sum: i32 = a.iter().sum();
    let mx = *a.iter().max().unwrap();

    let ans = mx.max(2 * sum / n as i32 + 1);
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
