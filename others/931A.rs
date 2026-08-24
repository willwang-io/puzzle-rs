// Created: Aug 24 2026, 16:49:49
// Formatted with rustfmt.

fn main() {
    let d = (read::<i32>() - read::<i32>()).abs();
    let x = d / 2;
    let y = d - x;
    let ans = x * (x + 1) / 2 + y * (y + 1) / 2;
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
