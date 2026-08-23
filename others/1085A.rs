// Created: Aug 22 2026, 22:37:28
// Formatted with rustfmt.

fn main() {
    let t = read::<String>().into_bytes();
    let n = t.len();
    let odd = (n + 1) / 2;
    let mut s = vec![];

    for i in 1..=n {
        let j = if i % 2 == 1 {
            odd - (i + 1) / 2
        } else {
            odd + i / 2 - 1
        };
        s.push(t[j]);
    }

    let ans = String::from_utf8(s).unwrap();
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
