use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;

pub fn nargo_compile(file_path: &PathBuf) -> anyhow::Result<String> {
    let output = Command::new("nargo")
        .arg("compile")
        .current_dir(file_path.parent().unwrap())
        .output()
        .context("Failed to run nargo compile")?;

    if !output.status.success() {
        anyhow::bail!(
            "Compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn nargo_execute(file_path: &PathBuf) -> anyhow::Result<String> {
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir(file_path.parent().unwrap())
        .output()
        .context("Failed to run nargo execute")?;

    if !output.status.success() {
        anyhow::bail!(
            "Execution failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn nargo_test(file_path: &PathBuf) -> anyhow::Result<String> {
    let output = Command::new("nargo")
        .arg("test")
        .current_dir(file_path.parent().unwrap())
        .output()
        .context("Failed to run nargo test")?;

    if !output.status.success() {
        anyhow::bail!("Tests failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn check_nargo_installation() -> bool {
    Command::new("nargo")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
