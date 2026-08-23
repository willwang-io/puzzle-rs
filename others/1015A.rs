// Created: Aug 23 2026, 11:44:34
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut covered = vec![false; m + 1];

    for _ in 0..n {
        let l: usize = read();
        let r: usize = read();
        covered[l..=r].fill(true);
    }

    let a: Vec<usize> = (1..=m).filter(|&i| !covered[i]).collect();
    let len = a.len();
    println!("{len}");

    if !a.is_empty() {
        let ans = a
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{ans}");
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
