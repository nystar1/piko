pub fn add(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    from_num(left_num.saturating_add(right_num))
}

pub fn sub(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    from_num(left_num.saturating_sub(right_num).max(1))
}

pub fn mul(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    from_num(left_num.saturating_mul(right_num))
}

pub fn div(left: &str, right: &str) -> String {
    let left_num = to_num(left);
    let right_num = to_num(right);
    if right_num == 0 {
        return "a".to_string();
    }
    from_num((left_num / right_num).max(1))
}

macro_rules! compare_op {
    ($name:ident, $op:tt) => {
        pub fn $name(left: &str, right: &str) -> bool {
            to_num(left) $op to_num(right)
        }
    };
}

compare_op!(compare_eq, ==);
compare_op!(compare_ne, !=);
compare_op!(compare_lt, <);
compare_op!(compare_gt, >);
compare_op!(compare_le, <=);
compare_op!(compare_ge, >=);

fn to_num(s: &str) -> u64 {
    s.chars()
        .map(|c| c as u64 - b'a' as u64 + 1)
        .fold(0, |acc, digit| acc * 26 + digit)
}

fn from_num(num: u64) -> String {
    if num == 0 {
        return "a".to_string();
    }
    
    let mut result = String::new();
    let mut n = num;
    
    while n > 0 {
        n -= 1;
        result.push(char::from(b'a' + (n % 26) as u8));
        n /= 26;
    }
    
    result.chars().rev().collect()
}
