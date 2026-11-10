fn main() {
    // include_dir embeds files at compile time. Track directory membership too,
    // so a newly converted note is included without requiring a clean build.
    println!("cargo:rerun-if-changed=../../builtin");
    println!("cargo:rerun-if-changed=../../assets/fonts");
    compact_json(
        "../../builtin/dnd2024-definitions/contracts.json",
        "contracts.json",
    );
    compact_json(
        "../../builtin/dnd2024-definitions/coverage.json",
        "coverage.json",
    );
}

/// Embed a copy of a large JSON file without the layout whitespace. Startup reads it
/// byte by byte, and most of the pretty-printed file is indentation.
fn compact_json(source: &str, name: &str) {
    let text = std::fs::read(source).expect("built-in JSON is readable");
    let mut out = Vec::with_capacity(text.len());
    let (mut quoted, mut escaped) = (false, false);
    for byte in text {
        if quoted {
            out.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quoted = false;
            }
        } else if !matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
            quoted = byte == b'"';
            out.push(byte);
        }
    }
    let target =
        std::path::Path::new(&std::env::var("OUT_DIR").expect("cargo sets OUT_DIR")).join(name);
    std::fs::write(target, out).expect("OUT_DIR is writable");
}
