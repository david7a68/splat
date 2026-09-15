fn main() {
    // include_dir embeds files at compile time. Track directory membership too,
    // so a newly converted note is included without requiring a clean build.
    println!("cargo:rerun-if-changed=builtin");
}
