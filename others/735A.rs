// Created: Aug 24 2026, 17:06:24
// Formatted with rustfmt.

fn main() {
    read::<usize>();
    let k: usize = read();
    let s = read::<String>().into_bytes();
    let g = s.iter().position(|&c| c == b'G').unwrap();
    let t = s.iter().position(|&c| c == b'T').unwrap();
    let d = g.abs_diff(t);
    let mut ok = d % k == 0;

    if ok {
        for i in 1..d / k {
            let p = if g < t { g + i * k } else { g - i * k };

            if s[p] == b'#' {
                ok = false;
                break;
            }
        }
    }

    if ok {
        println!("YES");
    } else {
        println!("NO");
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
