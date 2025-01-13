use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/s0_lexer/lexer.rs");
    println!("cargo:rerun-if-changed=src/s1_parser/modelica.lalrpop");
    println!("cargo:rerun-if-changed=src/s1_parser/ast.rs");
    lalrpop::process_root().unwrap();

    // Attempt to retrieve the current Git version
    let output = Command::new("git")
        .args(["describe", "--dirty", "--tags"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        // Convert the hash to a String and trim it
        let git_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Pass the Git hash to your Rust code via an environment variable
        println!("cargo:rustc-env=GIT_HASH={}", git_hash);

        // Optionally, display a warning during the build with the hash
        println!("cargo:warning=Using Git version: {}", git_hash);
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
