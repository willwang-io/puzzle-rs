// Created: Aug 24 2026, 17:08:57
// Formatted with rustfmt.

fn main() {
    let p: usize = read();
    let n: usize = read();
    let mut used = vec![false; p];

    for i in 1..=n {
        let x = read::<usize>() % p;

        if used[x] {
            println!("{i}");
            return;
        }

        used[x] = true;
    }

    println!("-1");
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
