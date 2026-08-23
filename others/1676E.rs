// Created: Aug 23 2026, 12:32:12
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let q: usize = read();
    let mut a: Vec<i64> = (0..n).map(|_| read()).collect();

    a.sort_by_key(|&x| std::cmp::Reverse(x));
    let mut pa = vec![0; n + 1];
    for i in 1..=n {
        pa[i] += a[i - 1] + pa[i - 1];
    }

    let mut ans = vec![];

    for _ in 0..q {
        let y: i64 = read();
        let tmp = pa.partition_point(|&x| x < y);
        if tmp == n + 1 {
            ans.push(String::from("-1"));
        } else {
            ans.push(tmp.to_string());
        }
    }

    println!("{}", ans.join(" "));
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

