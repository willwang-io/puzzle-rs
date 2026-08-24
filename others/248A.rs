// Created: Aug 24 2026, 17:11:20
// Formatted with rustfmt.

fn main() {
    let n: i32 = read();
    let mut left = 0;
    let mut right = 0;

    for _ in 0..n {
        left += read::<i32>();
        right += read::<i32>();
    }

    let ans = left.min(n - left) + right.min(n - right);
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
