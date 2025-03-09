pub mod bit_utils;
pub mod gates;
pub mod key;
pub mod mulfft;
pub mod params;
pub mod tlwe;
pub mod trgsw;
pub mod trlwe;
pub mod utils;

mod spqlios;
mod context;

#[cfg(target_os = "none")]
mod custom_getrandom {
    pub fn getrandom(buf: &mut [u8]) -> Result<(), ()> {
        // Implement a stub that returns zeros (or a real entropy source)
        for byte in buf.iter_mut() {
            *byte = 0;
        }
        Ok(())
    }
}
