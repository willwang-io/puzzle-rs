// Created: Aug 27 2026, 17:24:56
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let sum = a.iter().sum::<i32>();
    let prime = (2..).take_while(|x| x * x <= sum).all(|x| sum % x != 0);
    let skip = if prime {
        a.iter().position(|x| x % 2 == 1)
    } else {
        None
    };
    let ans: Vec<String> = (0..n)
        .filter(|&i| Some(i) != skip)
        .map(|i| (i + 1).to_string())
        .collect();
    println!("{}\n{}", ans.len(), ans.join(" "));
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
