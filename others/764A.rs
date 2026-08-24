// Created: Aug 24 2026, 17:01:17
// Formatted with rustfmt.

fn main() {
    let n: i32 = read();
    let m: i32 = read();
    let z: i32 = read();
    let mut a = n;
    let mut b = m;

    while b != 0 {
        (a, b) = (b, a % b);
    }

    let ans = z / (n / a * m);
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
