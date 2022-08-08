fn eq_with_nan_eq(a: f64, b: f64) -> bool {
    (a.is_nan() && b.is_nan()) || (a == b)
}

pub fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| eq_with_nan_eq(*a,*b))
}
