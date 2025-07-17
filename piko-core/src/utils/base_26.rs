pub fn add(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    match left_num.checked_add(right_num) {
        Some(result) => from_num(result),
        None => from_num(u64::MAX),
    }
}

pub fn sub(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    let result = if left_num > right_num {
        left_num - right_num
    } else {
        1
    };
    from_num(result)
}

pub fn mul(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    match left_num.checked_mul(right_num) {
        Some(result) => from_num(result),
        None => from_num(u64::MAX),
    }
}

pub fn div(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    if right_num == 0 {
        return "a".to_string();
    }
    let result = left_num / right_num;
    if result == 0 {
        "a".to_string()
    } else {
        from_num(result)
    }
}

pub fn compare_eq(left: &str, right: &str) -> bool {
    to_num(left) == to_num(right)
}

pub fn compare_ne(left: &str, right: &str) -> bool {
    to_num(left) != to_num(right)
}

pub fn compare_lt(left: &str, right: &str) -> bool {
    to_num(left) < to_num(right)
}

pub fn compare_gt(left: &str, right: &str) -> bool {
    to_num(left) > to_num(right)
}

pub fn compare_le(left: &str, right: &str) -> bool {
    to_num(left) <= to_num(right)
}

pub fn compare_ge(left: &str, right: &str) -> bool {
    to_num(left) >= to_num(right)
}

fn to_num(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| {
        acc * 26 + (c as u64 - 'a' as u64 + 1)
    })
}

fn from_num(mut num: u64) -> String {
    if num == 0 {
        return "a".to_string();
    }
    let mut result = String::new();
    while num > 0 {
        num -= 1;
        result.push((b'a' + (num % 26) as u8) as char);
        num /= 26;
    }
    result.chars().rev().collect()
}
