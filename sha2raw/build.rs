extern crate cc;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let mut build512 = cc::Build::new();
    let sha512_path = if target_arch == "x86_64" {
        if cfg!(feature = "avx") {
            "src/sha512-avx-asm.S"
        }
        else if cfg!(feature = "avx2") {
            "src/sha512-avx2-asm.S"
        }
        else {
            return;
        }
    }else {
        panic!("Unsupported target architecture");
    };

    if target_arch == "x86_64" {
        build512
                  .no_default_flags(true)
                  .file(sha512_path)
                  .flag("-c")
                  .flag("-DHAS_AVX")
                  .pic(true)
                  .static_flag(true)
                  .shared_flag(false)
                  .compile("libsha512_avx_asm.a");
    }
}