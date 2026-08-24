// Created: Aug 24 2026, 16:29:53
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: usize = read();
    let b: usize = read();

    let mut p = vec![2; n];

    for _ in 0..a {
        p[read::<usize>() - 1] = 1;
    }

    for _ in 0..b {
        read::<usize>();
    }

    let ans = p
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
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
