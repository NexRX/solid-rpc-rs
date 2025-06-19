use std::process::Command;

fn main() {
    if std::fs::exists("dist").is_ok_and(|v| v) {
        println!("cargo:warning=Spa build detected, skipping build");
        return
    }
    
    let manager = js_package_manager();
    println!("cargo:warning=Using {} as package manager", manager);

    // Run the build command
    let status = Command::new(manager)
        .arg("run")
        .arg("build")
        .status()
        .expect("Failed to execute build command");

    if !status.success() {
        panic!("Build command failed with status: {}", status);
    }
}

fn js_package_manager() -> &'static str {
    ["pnpm", "bun", "deno", "yarn", "npm"]
        .iter()
        .find_map(|manager| {
            Command::new(manager)
                .arg("--version")
                .output()
                .map(|_| manager)
                .ok()
        })
        .expect("no JavaScript package manager installation found")
}
