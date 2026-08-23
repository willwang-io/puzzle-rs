// Created: Aug 22 2026, 12:23:49
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let a = read::<i32>() - 1;
    let t: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut ans = 0;

    for i in 0..n {
        let j = 2 * a - i as i32;

        if t[i] == 1 && (j < 0 || j >= n as i32 || t[j as usize] == 1) {
            ans += 1;
        }
    }

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
