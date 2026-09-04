// Created: Sep  4 2026, 16:33:41
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut a: Vec<usize> = (0..n).map(|_| read()).collect();

    'outer: for i in 1..=n {
        if a[i - 1] == i {
            continue;
        }
        for j in i..=n {
            if a[j - 1] == i {
                a[i - 1..j].reverse();
                break 'outer;
            }
        }
    }

    let ans = a
        .iter()
        .map(|&x| x.to_string())
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
