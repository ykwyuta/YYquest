pub(crate) fn hash(x: i32, y: i32) -> u32 {
    let v = (x as u32).wrapping_mul(0x9e37_79b9) ^ (y as u32).wrapping_mul(0x85eb_ca6b);
    let v = (v ^ (v >> 16)).wrapping_mul(0x7feb_352d);
    v ^ (v >> 15)
}

pub(crate) fn sample(x: i32, y: i32, length: i32) -> i32 {
    let (ox, oy) = (x.div_euclid(length), y.div_euclid(length));
    let (tx, ty) = (x.rem_euclid(length), y.rem_euclid(length));
    let corner = |x, y| (hash(x, y) & 255) as i32;
    let lerp = |a, b, f| (a * (length - f) + b * f) / length;
    lerp(
        lerp(corner(ox, oy), corner(ox + 1, oy), tx),
        lerp(corner(ox, oy + 1), corner(ox + 1, oy + 1), tx),
        ty,
    )
}
