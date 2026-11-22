fn main() {
    assemble_bundled_engine();
    tauri_build::build()
}

/// The Windows engine archive is kept as small parts in git because GitHub
/// does not allow LFS objects on public forks and a single file is over 100 MB.
/// Rebuild the exact archive before Rust reads it with include_bytes!().
fn assemble_bundled_engine() {
    use std::fs;
    use std::path::Path;

    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("bundled");
    let zip = dir.join("ShardX-Windows.zip");
    let parts_dir = dir.join("runtime-parts");

    if zip.is_file() && fs::metadata(&zip).map(|m| m.len() > 0).unwrap_or(false) {
        return;
    }

    let mut names: Vec<_> = fs::read_dir(&parts_dir)
        .expect("bundled engine parts folder")
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("ShardX-Windows.zip."))
        })
        .collect();
    names.sort();

    assert!(!names.is_empty(), "no bundled engine parts in src-tauri/bundled/runtime-parts");
    let mut out = Vec::new();
    for p in &names {
        out.extend_from_slice(&fs::read(p).unwrap_or_else(|e| panic!("read {}: {e}", p.display())));
    }
    assert!(!out.is_empty(), "empty bundled engine archive");
    fs::write(&zip, &out).expect("write bundled engine archive");
    println!("cargo:rerun-if-changed=bundled/runtime-parts");
}
