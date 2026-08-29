// Created: Aug 29 2026, 11:23:15
// Formatted with rustfmt.

fn solve() {
    let h: usize = read();
    let w: usize = read();
    let mut a = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if i != 0 && i + 1 != h && j != 0 && j + 1 != w {
                continue;
            }
            let ok = (i.saturating_sub(1)..=(i + 1).min(h - 1))
                .all(|x| (j.saturating_sub(1)..=(j + 1).min(w - 1)).all(|y| a[x][y] == 0));
            if ok {
                a[i][j] = 1;
            }
        }
    }
    for row in a {
        let ans: String = row.into_iter().map(|x| (b'0' + x) as char).collect();
        println!("{ans}");
    }
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
