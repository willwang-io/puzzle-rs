// Created: Aug 30 2026, 15:52:44
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut cnt = vec![0; 101];
    for _ in 0..n {
        cnt[read::<usize>()] += 1;
    }
    let mut a = vec![];
    for x in 0..=100 {
        if cnt[x] > 0 {
            a.push(x);
        }
    }
    for x in 0..=100 {
        for _ in 1..cnt[x] {
            a.push(x);
        }
    }
    let ans = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
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
