// Created: Aug 30 2026, 15:29:53
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let x: i32 = read();
    let mut a: Vec<i32> = (0..n).map(|_| read()).collect();
    if a.iter().sum::<i32>() == x {
        println!("NO");
        return;
    }
    a.sort_unstable();
    let mut sum = 0;
    for i in 0..n {
        if sum + a[i] == x {
            a.swap(i, n - 1);
        }
        sum += a[i];
    }
    let ans = a
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("YES\n{ans}");
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
