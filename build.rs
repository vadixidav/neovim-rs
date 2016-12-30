use std::process::Command;
use std::path::PathBuf;

#[cfg(target_os="macos")]
fn print_lib_dir() {
    println!("cargo:rustc-flags=-L /usr/local/opt/gettext/lib/");
}

#[cfg(not(target_os="macos"))]
fn print_lib_dir() {}

fn main() {
    let curr_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .current_dir(&curr_dir)
        .status()
        .unwrap();

    let nvim_dir = curr_dir.join("neovim");
    Command::new("make").arg("deps").current_dir(&nvim_dir).status().unwrap();
    Command::new("make").arg("libnvim").current_dir(&nvim_dir).status().unwrap();

    let nvim_lib_dir = nvim_dir.join("build").join("lib");
    let deps_lib_dir = nvim_dir.join(".deps").join("usr").join("lib");
    println!("cargo:rustc-flags=-L {} -L {} -l nvim",
             nvim_lib_dir.to_str().unwrap(),
             deps_lib_dir.to_str().unwrap());

    print_lib_dir();
}