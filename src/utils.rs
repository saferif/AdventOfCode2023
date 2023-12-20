fn gcd(x: u64, y: u64) -> u64 {
    let z = x | y;
    if x == 0 || y == 0 {
        return z;
    }
    let e = z.trailing_zeros();
    let mut x = x >> x.trailing_zeros();
    let mut y = y >> y.trailing_zeros();
    while x != y {
        if x < y {
            core::mem::swap(&mut x, &mut y);
        }
        x -= y;
        x >>= x.trailing_zeros();
    }
    x << e
}

pub(crate) fn lcm(x: u64, y: u64) -> u64 {
    x / gcd(x, y) * y
}
