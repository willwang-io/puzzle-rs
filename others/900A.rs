// Created: Aug 24 2026, 16:59:18
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut right = 0;
    let mut left = 0;

    for _ in 0..n {
        if read::<i32>() < 0 {
            left += 1;
        } else {
            right += 1;
        }
        read::<i32>();
    }

    if left <= 1 || right <= 1 {
        println!("YES");
    } else {
        println!("NO");
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
