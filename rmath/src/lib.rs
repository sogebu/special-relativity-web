mod angle;
mod matrix;
mod quaternion;
mod vector;

pub use angle::*;
pub use matrix::*;
pub use quaternion::*;
pub use vector::*;

/// Maybe useful at WebGL
///
/// ```rust
/// # use rmath::{Vector3, as_f32_as_bytes};
/// let v = vec![Vector3::new(0.0, 1.0, -3.0)];
/// assert_eq!(
///     as_f32_as_bytes(&v),
///     vec![
///         0x00, 0x00, 0x00, 0x00,
///         0x00, 0x00, 0x80, 0x3f,  // 0 01111111 00000000000000000000000
///         0x00, 0x00, 0x40, 0xc0,  // 1 10000000 10000000000000000000000
///     ],
/// );
/// ```
pub fn as_f32_as_bytes(v: &[Vector3]) -> Vec<u8> {
    let mut floats = Vec::with_capacity(v.len() * 3);
    for v in v {
        floats.push(v.x as f32);
        floats.push(v.y as f32);
        floats.push(v.z as f32);
    }
    // This is safe?
    let bytes = unsafe {
        Vec::from_raw_parts(
            floats.as_ptr() as *mut u8,
            floats.len() * std::mem::size_of::<f32>(),
            floats.len() * std::mem::size_of::<f32>(),
        )
    };
    std::mem::forget(floats);
    bytes
}
