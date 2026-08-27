// Created: Aug 26 2026, 21:07:11
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    let mut seen = false;
    let mut ended = false;
    let mut ok = true;

    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            if ended {
                ok = false;
            }
            seen = true;
        } else if seen {
            ended = true;
        }
    }

    if ok {
        println!("YES");
    } else {
        println!("NO");
    }
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
