// Created: Aug 23 2026, 12:26:55
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let mut cnt = 0;
    let mut a = vec![];

    let mut ans = String::new();

    for i in 0..n {
        let x: i32 = read();
        if a.is_empty() {
            ans.push_str("1");
            a.push(x);
        } else {
            let tmp = cnt + if *a.last().unwrap() > x { 1 } else { 0 };

            let c = if tmp == 0 || (tmp == 1 && x <= a[0]) {
                a.push(x);
                cnt = tmp;
                "1"
            } else {
                "0"
            };

            ans.push_str(c);
        }
    }

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
