// Created: Aug 29 2026, 11:19:46
// Formatted with rustfmt.

fn solve() {
    let n: usize = read();
    let m: usize = read();
    let a: Vec<Vec<u8>> = (0..n).map(|_| read::<String>().into_bytes()).collect();
    let flip = (0..2).find(|&x| {
        (0..n).all(|i| {
            (0..m).all(|j| {
                let c = if (i + j + x) % 2 == 0 { b'R' } else { b'W' };
                a[i][j] == b'.' || a[i][j] == c
            })
        })
    });
    let Some(flip) = flip else {
        println!("NO");
        return;
    };
    println!("YES");
    for i in 0..n {
        let row: String = (0..m)
            .map(|j| if (i + j + flip) % 2 == 0 { 'R' } else { 'W' })
            .collect();
        println!("{row}");
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
