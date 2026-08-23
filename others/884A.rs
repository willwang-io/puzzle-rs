// Created: Aug 23 2026, 11:42:52
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let mut t: i32 = read();

    for i in 1..=n {
        t -= 86400 - read::<i32>();

        if t <= 0 {
            println!("{i}");
            return;
        }
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
