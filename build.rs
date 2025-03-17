fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=web");
    println!("cargo:rerun-if-changed=tailwind.config.ts");

    let path = std::fs::canonicalize(".").expect("failed geting path");

    // Build Tailwind
    std::process::Command::new("bunx")
        .current_dir(path.clone())
        .args([
            "@tailwindcss/cli",
            "-i",
            "./web/tailwind.css",
            "-o",
            "./static/tailwind.css",
        ])
        .spawn()
        .expect("failed running tailwind");

    // Build first-party TypeScript
    std::process::Command::new("bun")
        .current_dir(path.clone())
        .args(["build", "./web/base.ts", "--outdir", "./static", "--minify"])
        .spawn()
        .expect("failed building base.ts");

    // Build vendor packages
    let vendor_dir = path.join("web/static/vendor");
    std::fs::create_dir_all(&vendor_dir).expect("Failed to create vendor directory");
    let vendor_files = ["node_modules/@starfederation/datastar/dist/bundles/datastar.js"];

    for file in vendor_files.iter() {
        std::process::Command::new("bun")
            .current_dir(&path)
            .args(["build", file, "--outdir", "./static/vendor", "--minify"])
            .status()
            .expect(&format!("failed building vendor file: {}", file));
    }

    // Cache bust static files
    let asset_dirs = vec![path.join("./static")];
    let out_dir = path.join("./src/statics/assets.rs");
    cacheb::codegen(&out_dir, &asset_dirs, &[]).unwrap();
}
