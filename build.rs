fn main() {
    println!("cargo:rerun-if-changed=src/umash.c");
    println!("cargo:rerun-if-changed=src/umash_long.inc");
    println!("cargo:rerun-if-changed=src/umash.h");

    cc::Build::new()
        .file("src/umash.c")
        .opt_level(2)
        .flag_if_supported("-mpclmul")
        .flag_if_supported("-march=armv8-a+crypto")
        .compile("umash")
}
