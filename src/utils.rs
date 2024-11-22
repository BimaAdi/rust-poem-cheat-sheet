pub fn div_ceil(x: i32, y: i32) -> i32 {
    let x_u = x as u32;
    let y_u = y as u32;
    x_u.div_ceil(y_u) as i32
}
