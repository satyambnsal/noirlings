use anyhow::Context;
use std::path::PathBuf;
use std::process::Command;
use std::{env::current_dir, fs};

// Prepares testing crate
// Copies the exercise file into testing crate
pub fn prepare_crate_for_exercise(file_path: &PathBuf) -> PathBuf {
    let crate_path = current_dir().unwrap().join(PathBuf::from("runner-crate"));
    let src_dir = crate_path.join("src");
    if !src_dir.exists() {
        let _ = fs::create_dir(&src_dir);
    }
    let lib_path = src_dir.join("main.nr");
    let file_path = current_dir().unwrap().join(file_path);

    match fs::copy(&file_path, &lib_path) {
        Ok(_) => {}
        Err(err) => panic!("Error occurred while preparing the exercise,\nExercise: {file_path:?}\nLib path: {lib_path:?}\n{err:?}"),
    };
    crate_path
}

// pub fn nargo_config(crate_path: PathBuf) -> Config {
//     let path = Utf8PathBuf::from_path_buf(crate_path.join(PathBuf::from("Nargo.toml"))).unwrap();

//     Config::builder(path).build().unwrap()
// }

pub fn nargo_compile(file_path: &PathBuf) -> anyhow::Result<String> {
    let crate_path = prepare_crate_for_exercise(file_path);
    let output = Command::new("nargo")
        .arg("compile")
        .current_dir(crate_path)
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
    let crate_path = prepare_crate_for_exercise(file_path);
    let output = Command::new("nargo")
        .arg("execute")
        .current_dir(crate_path)
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
    let crate_path = prepare_crate_for_exercise(file_path);
    let output = Command::new("nargo")
        .arg("test")
        .arg("--show-output")
        .current_dir(crate_path)
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
