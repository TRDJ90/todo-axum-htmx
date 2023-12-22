fn main() {
    println!("cargo:rerun-if-changed=templates/");

    let dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let input = format!("{dir}/input.css");
    let output = format!("{dir}/static/dist.css");

    let result = std::process::Command::new("npx")
        .args(["--yes", "tailwindcss", "-i", &input, "-o", &output])
        .output()
        .expect("Unable to generate css");

    if !result.status.success() {
        let error = String::from_utf8_lossy(&result.stderr);
        println!("cargo:warning=Unable to build CSS!");
        println!("cargo:warning=Output: {error}");
    }

    println!("cargo:warning=Generated css");
}
