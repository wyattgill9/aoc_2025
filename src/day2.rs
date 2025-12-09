use std::fs;

fn repeats_1(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    let (l, r) = s.split_at(len / 2);
    l == r
}

fn repeats_2(num: u64) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for pattern_len in 1..=(len / 2) {
        if len % pattern_len == 0 {
            if bytes.chunks(pattern_len).all(|chunk| chunk == &bytes[..pattern_len]) {
                return true;
            }
        }
    }

    false
}

fn _impl(repeats: fn(u64) -> bool, path: &str) -> u64 {
    fs::read_to_string(path).unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once('-')?;
            let l = left.parse().ok()?;
            let r = right.parse().ok()?;

            Some((l..=r)
                .filter(|&i| repeats(i))
                .sum::<u64>())
        })
        .sum()
}

pub fn day2_1(path: &str) -> u64 {
    _impl(repeats_1, path)
}

pub fn day2_2(path: &str) -> u64 {
    _impl(repeats_2, path)
}
