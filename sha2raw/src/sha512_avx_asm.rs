//! Assembly implementation of [SHA-512-AVX] compression functions.
//!
//! Only x86-64 architectures are currently supported.

#[cfg(not(any(target_arch = "x86_64")))]
compile_error!("crate can only be used on x86-64 architectures");

//#[feature(global_asm)]
//global_asm!(include_str!("sha512_avx_asm.S"));

use std::ffi::c_void;

#[link(name="sha512_avx_asm", kind="static")]
extern "C" {
    //# void sha512_with_avx(const void* M, void* D, u64 L)
    fn sha512_with_avx(msg: *const c_void, state: *mut c_void, len: u64);
}

/// Safe wrapper around assembly implementation of SHA512-AVX compression function
#[inline]
pub fn compress512_avx(msg: *const c_void, state: *mut c_void, len: u64) {
    unsafe { sha512_with_avx(msg, state, len) }
}
