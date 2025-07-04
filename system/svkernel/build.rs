use std::env;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let linker_script = match arch.as_str() {
        "riscv64" => Some("riscv64.ld"),
        _ => None
    };

    if let Some(linker_script) = linker_script {
        println!("cargo::rustc-link-arg=-T{}/src/arch/{}/{}", manifest_dir, arch, linker_script);
    }
}
