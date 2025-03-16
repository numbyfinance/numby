use std::path::PathBuf;

fn main() {
    let path = std::fs::canonicalize(".");

    let path = match path {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    // Build Tailwind
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=web/tailwind.css");
    println!("cargo::rerun-if-changed=tailwind.config.ts");
    std::process::Command::new("sh")
        .current_dir(path.clone())
        .arg("-c")
        .arg("bunx @tailwindcss/cli -i ./web/tailwind.css -o ./web/static/tailwind.css")
        .spawn()
        .expect("failed running tailwind");

    // Build first-party TypeScript
    println!("cargo::rerun-if-changed=web/base.ts");
    std::process::Command::new("sh")
        .current_dir(path.clone())
        .arg("-c")
        .arg("bun build web/base.ts --outdir web/static --minify")
        .spawn()
        .expect("failed building base.ts");

    // Build vendor packages
    let vendor_dir = path.join("web/static/vendor");
    std::fs::create_dir_all(&vendor_dir).expect("Failed to create vendor directory");
    let vendor_files = ["node_modules/@starfederation/datastar/dist/bundles/datastar.js"];

    for file in vendor_files.iter() {
        std::process::Command::new("bun")
            .current_dir(&path)
            .arg("build")
            .arg(file)
            .arg("--outdir")
            .arg("web/static/vendor")
            .arg("--minify")
            .status()
            .expect(&format!("failed building vendor file: {}", file));
    }

    // Cache bust static files
    let asset_dirs = vec![PathBuf::from("./web/static")];
    let out_dir = PathBuf::from("./src/static/file.rs");

    cacheb::codegen(&out_dir, &asset_dirs, &[]).unwrap();
}
