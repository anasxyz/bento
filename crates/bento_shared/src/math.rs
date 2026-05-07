pub fn transform(rotate: f32, scale_x: f32, scale_y: f32) -> [f32; 4] {
    let cos = rotate.cos();
    let sin = rotate.sin();
    [
        cos * scale_x,
        sin * scale_x,
        -sin * scale_y,
        cos * scale_y,
    ]
}
