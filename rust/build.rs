fn main() {
    let mut opts = built::Options::default();
    opts.set_env(true);
    opts.set_git(true);
    opts.set_time(true);

    let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("build_info.rs");
    built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
        .expect("Failed to acquire build-time information");
}
