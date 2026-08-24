// Created: Aug 24 2026, 16:37:23
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut has = [false; 10];

    for _ in 0..m {
        has[read::<usize>()] = true;
    }

    let ans = a
        .into_iter()
        .filter(|&x| has[x as usize])
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
