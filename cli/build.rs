use std::process::Command;
use std::path::Path;
use std::fs;

fn main() {
    // Copy VERSION file. Do not fail e.g. when built via `cargo publish`
    //println!("cargo:rerun-if-changed=../VERSION");

    // Fetch current git hash to print version output
    let git_version_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("should run 'git rev-parse HEAD' to get git hash");
    let git_hash = String::from_utf8(git_version_output.stdout)
        .expect("should read 'git' stdout to find hash");
    // Assign the git hash to the compile-time GIT_HASH env variable (to use with env!())
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let version_file = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"));
    let version = fs::read_to_string(&version_file).expect("Unable to read VERSION file");
    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version);

    // Fetch OS information if on unix
    if cfg!(unix) {
        let get_uname_options = if cfg!(target_os = "macos") {
            "-srm"
        } else {
            "-srom"
        };
        let options = vec![get_uname_options];
        let uname_output = Command::new("uname")
            .args(&options)
            .output()
            .expect("should get OS info from uname");
        let uname_text =
            String::from_utf8(uname_output.stdout).expect("should read uname output to string");
        println!("cargo:rustc-env=UNAME={}", uname_text);
    }
}