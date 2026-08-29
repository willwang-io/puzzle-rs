// Created: Aug 29 2026, 12:26:06
// Formatted with rustfmt.

fn solve() {
    let px: i32 = read();
    let py: i32 = read();
    let s: String = read();

    let mut u = 0;
    let mut d = 0;
    let mut r = 0;
    let mut l = 0;

    for c in s.chars() {
        match c {
            'U' => u += 1,
            'D' => d += 1,
            'R' => r += 1,
            'L' => l += 1,
            _ => {}
        }
    }

    let ok_x = if px >= 0 { r >= px } else { l >= -px };
    let ok_y = if py >= 0 { u >= py } else { d >= -py };

    if ok_x && ok_y {
        println!("YES");
    } else {
        println!("NO");
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
