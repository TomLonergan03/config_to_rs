const CONFIG_PATH: &str = "tests/test.yaml";

fn main() {
    println!("cargo:rerun-if-changed=.");
    println!("cargo:rerun-if-changed={}", CONFIG_PATH);

    println!("cargo:rustc-env=CONFIG_PATH={}", CONFIG_PATH);
}
