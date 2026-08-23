// Created: Aug 23 2026, 12:02:32
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut x = read::<String>().into_bytes();
    let y = read::<String>().into_bytes();

    let mut ans = 0;

    for i in 0..n {
        if y[i] == b'1' {
            if x[i] == b'0' {
                ans += 1;
                x[i] = b'x';
            } else if i >= 1 && x[i - 1] == b'1' {
                ans += 1;
                x[i - 1] = b'x';
            } else if i + 1 < n && x[i + 1] == b'1' {
                ans += 1;
                x[i + 1] = b'x';
            }
        }
    }

    println!("{ans}");
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        solve();
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
