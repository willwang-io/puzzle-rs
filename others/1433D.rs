// Created: Aug 23 2026, 12:21:25
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let a: Vec<i32> = (0..n).map(|_| read()).collect();
    let mut ans = vec![];
    let mut j = None;
    for i in 1..n {
        if a[i] != a[0] {
            j = Some(i);
            ans.push((1, i + 1));
        }
    }

    if j.is_none() {
        println!("NO");
        return;
    }

    println!("YES");
    for i in 1..n {
        if a[i] == a[0] {
            ans.push((j.unwrap() + 1, i + 1));
        }
    }

    let ans = ans
        .iter()
        .map(|&x| format!("{} {}", x.0, x.1))
        .collect::<Vec<_>>()
        .join("\n");
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
