//! Assembly implementation of [SHA-512-AVX] compression functions.
//!
//! Only x86-64 architectures are currently supported.

#[cfg(not(any(target_arch = "x86_64")))]
compile_error!("crate can only be used on x86-64 architectures");

//#[feature(global_asm)]
//global_asm!(include_str!("sha512_avx_asm.S"));

use std::ffi::c_void;

#[cfg(any(feature = "sha512_avx", feature = "sha512_avx2"))]
#[link(name="sha512_avx_asm", kind="static")]
extern "C" {
    //# void sha512_transform_avx(sha512_state *state, const u8 *data, int blocks)
    fn sha512_transform_avx(state: *mut c_void, data: *const c_void, blocks: i32);
}

/// Safe wrapper around assembly implementation of SHA512-AVX compression function
#[cfg(any(feature = "sha512_avx", feature = "sha512_avx2"))]
#[inline]
pub fn compress512_avx(msg: *const c_void, state: *mut c_void, len: i32) {    
    unsafe { sha512_transform_avx(state, msg, len) }
}
