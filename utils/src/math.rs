// From https://doc.rust-lang.org/std/ops/trait.Div.html
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
pub fn greatest_common_divisor(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

pub fn lowest_common_multiple_3(a: u64, b: u64, c: u64) -> u64 {
    lowest_common_multiple(a, lowest_common_multiple(b, c))
}

pub fn lowest_common_multiple_many(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let others = lowest_common_multiple_many(&nums[1..]);
        nums[0] * others / greatest_common_divisor(nums[0], others)
    }
}
