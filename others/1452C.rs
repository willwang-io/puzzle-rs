// Created: Aug 29 2026, 12:19:44
// Formatted with rustfmt.

fn solve() {
    let s: String = read();
    let mut ans = 0;
    let mut stk1 = vec![];
    let mut stk2 = vec![];

    for c in s.chars() {
        match c {
            '(' => stk1.push(c),
            '[' => stk2.push(c),
            ')' => {
                if stk1.pop() == Some('(') {
                    ans += 1;
                }
            }
            ']' => {
                if stk2.pop() == Some('[') {
                    ans += 1;
                }
            }
            _ => {}
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
