extern crate cc;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if target_arch != "x86_64" {
        panic!("Unsupported target architecture");
    }

    #[cfg(all(feature = "sha512_avx", feature = "sha512_avx2"))]
    painc("Can not use feature \"sha512_avx\" and \"sha512_avx2\" at the same time!");

    #[cfg(any(feature = "sha512_avx", feature = "sha512_avx2"))]
    {
        #[allow(unused)]
        let mut sha512_path = "";

        #[cfg(feature = "sha512_avx")]
        {
            println!("build sha2raw with avx!");
            sha512_path = "src/sha512-avx-asm.S";
        }

        #[cfg(feature = "sha512_avx2")]
        {
            println!("build sha2raw with avx2!");
            sha512_path = "src/sha512-avx2-asm.S";
        }

        cc::Build::new()
            .no_default_flags(true)
            .file(sha512_path)
            .flag("-c")
            .flag("-O3")
            .flag("-DHAS_AVX")
            .pic(true)
            .static_flag(true)
            .compile("libsha512_avx_asm.a");
    }
}
