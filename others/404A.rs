// Created: Aug 23 2026, 12:15:49
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();
    let x = a[0][0];
    let y = a[0][1];
    let mut ok = x != y;

    for i in 0..n {
        for j in 0..n {
            let expected = if i == j || i + j == n - 1 { x } else { y };
            ok &= a[i][j] == expected;
        }
    }

    let ans = if ok { "YES" } else { "NO" };
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

