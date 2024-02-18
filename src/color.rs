//! Module providing simple functions for generation and manipulation of colors.
use rand::Rng;
use tap::Tap;

/// Generates an array of random floats in [0.;1), which can be used to represent RBG colors.
pub fn rand_rbg() -> [f32; 3] {
    // Currently uses `rand`[0.8.5]`::distributions::Standard`.
    [f32::NAN; 3].tap_mut(|arr| rand::thread_rng().fill(arr))
}
/// Computes the contrast between two colors as
///  a sum of distances between respective RBG components.
///
/// That is |R1-R2|+|B1-B2|+|G1-G2|.
pub fn rbg_contrast(lhs: [f32; 3], rhs: [f32; 3]) -> f32 {
    lhs.into_iter().zip(rhs).map(|(x, y)| (x - y).abs()).sum()
}
/// Generates a random color, represented by 3 float values in [0;1.
///  Returned color will have constrast >= `min_contrast`.
///
/// `rbg_contrast` is used to determine the value of contrast between the two colors.
/// If requested `min_contrast` is very large (or unachievable) the function may deadlock.
pub fn contrasting_rand_rbg(prev: [f32; 3], min_contrast: f32) -> [f32; 3] {
    loop {
        let next = rand_rbg();
        if rbg_contrast(prev, next) >= min_contrast {
            return next;
        }
    }
}
