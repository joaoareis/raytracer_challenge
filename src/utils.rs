pub fn compare_float(a: &f32, b: &f32) -> bool {
    if (a - b).abs() < 0.0001 {
        true
    } else {
        false
    }
}

pub fn assert_float_eq(a: f32, b: f32){
    assert_eq!(compare_float(&a,&b), true)
}
