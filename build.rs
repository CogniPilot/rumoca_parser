use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/s0_lexer");
    println!("cargo:rerun-if-changed=src/s1_parser");
    lalrpop::Configuration::new()
        .emit_rerun_directives(true)
        .process_current_dir()
        .unwrap();

    // Attempt to retrieve the current Git version
    let output = Command::new("git")
        .args(["describe", "--dirty", "--tags", "--long"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        // Convert the hash to a String and trim it
        let git_ver = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Pass the Git hash to your Rust code via an environment variable
        println!("cargo:rustc-env=GIT_VER={}", git_ver);

        // Optionally, display a warning during the build with the hash
        println!("cargo:warning=Using Git version: {}", git_ver);
    } else {
        eprintln!(
            "Failed to retrieve Git version: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Rerun this build script if `.git/HEAD` or its references change
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
}
