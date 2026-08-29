// Created: Aug 29 2026, 12:41:50
// Formatted with rustfmt.

fn main() {
    let n: usize = read();
    let s = read::<String>().into_bytes();
    let mut all = vec![0; 128];
    for &c in &s {
        all[c as usize] += 1;
    }
    let need = all.iter().filter(|&&x| x > 0).count();
    let mut cnt = vec![0; 128];
    let mut have = 0;
    let mut left = 0;
    let mut ans = n;
    for right in 0..n {
        let c = s[right] as usize;
        if cnt[c] == 0 {
            have += 1;
        }
        cnt[c] += 1;
        while have == need {
            ans = ans.min(right - left + 1);
            let c = s[left] as usize;
            cnt[c] -= 1;
            if cnt[c] == 0 {
                have -= 1;
            }
            left += 1;
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
