// Created: Aug 24 2026, 16:47:25
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    let mut ans = n;
    let mut i = 0;

    while i + 1 < n {
        if s[i] != s[i + 1] {
            ans -= 1;
            i += 2;
        } else {
            i += 1;
        }
    }

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
