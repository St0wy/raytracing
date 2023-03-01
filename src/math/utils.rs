use std::f32::consts::PI;

/// Converts degrees to radians.
///
/// # Arguments
///
/// * `degrees`: Degrees to convert to radians.
///
/// returns: f32
///
/// # Examples
///
/// ```
/// use std::f32::consts::PI;
/// use raytracing::math::utils::degrees_to_radians;
/// assert_eq!(degrees_to_radians(360.0), PI * 2.0);
/// assert_eq!(degrees_to_radians(180.0), PI);
/// ```
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
