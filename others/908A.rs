// Created: Aug 24 2026, 16:54:46
// Formatted with rustfmt.

fn main() {
    let ans = read::<String>()
        .bytes()
        .filter(|&c| {
            matches!(
                c,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'1' | b'3' | b'5' | b'7' | b'9'
            )
        })
        .count();
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
