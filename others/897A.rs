// Created: Aug 23 2026, 11:36:48
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let m: usize = read();
    let mut s = read::<String>().into_bytes();

    for _ in 0..m {
        let l = read::<usize>() - 1;
        let r: usize = read();
        let c1: char = read();
        let c2: char = read();

        for i in l..r {
            if s[i] == c1 as u8 {
                s[i] = c2 as u8;
            }
        }
    }

    let ans = String::from_utf8(s).unwrap();
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
