fn main() {
    println!("cargo:rerun-if-changed=src/umash.c");
    println!("cargo:rerun-if-changed=src/umash_long.inc");
    println!("cargo:rerun-if-changed=src/umash.h");

    let long_inputs = if cfg!(feature = "long_input_routines") {
        "1"
    } else {
        "0"
    };

    cc::Build::new()
        .file("src/umash.c")
        .opt_level(2)
        .flag_if_supported("-mpclmul")
        .flag_if_supported("-march=armv8-a+crypto")
        .define("UMASH_LONG_INPUTS", long_inputs)
        .compile("umash");
}
