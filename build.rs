fn main() {
    let path = std::fs::canonicalize(".");

    let path = match path {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=tailwind.config.ts");
    std::process::Command::new("sh")
        .current_dir(path)
        .arg("-c")
        .arg("bunx @tailwindcss/cli -i ./web/input.css -o ./web/assets/tailwind.css")
        .spawn()
        .expect("failed running tailwind");
}
