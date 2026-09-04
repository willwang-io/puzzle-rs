// Created: Aug 31 2026, 10:41:16
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let q: usize = read();
    let mut a: Vec<usize> = (0..n).map(|_| read()).collect();
    let mut ones = a.iter().sum::<usize>();
    for _ in 0..q {
        let t: i32 = read();
        let x: usize = read();
        if t == 1 {
            ones -= a[x - 1];
            a[x - 1] ^= 1;
            ones += a[x - 1];
        } else {
            let ans = if x <= ones { 1 } else { 0 };
            println!("{ans}");
        }
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
