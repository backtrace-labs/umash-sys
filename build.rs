fn main() {
    println!("cargo:rerun-if-changed=src/umash.c");
    println!("cargo:rerun-if-changed=src/umash.h");

    cc::Build::new()
        .file("src/umash.c")
        .opt_level(2)
        .flag("-mpclmul")
        .compile("umash")
}
