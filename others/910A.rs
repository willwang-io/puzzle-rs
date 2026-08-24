// Created: Aug 24 2026, 16:51:48
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let d: usize = read();
    let s = read::<String>().into_bytes();
    let mut pos = 0;
    let mut ans = 0;

    while pos < n - 1 {
        let mut next = pos;

        for i in (pos + 1..=(pos + d).min(n - 1)).rev() {
            if s[i] == b'1' {
                next = i;
                break;
            }
        }

        if next == pos {
            println!("-1");
            return;
        }

        pos = next;
        ans += 1;
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
