use std::process::Command;

use crate::{
    exercise::{Exercise, Mode},
    utils,
};

// Run the exercise according to its mode
pub fn run(exercise: &Exercise) -> Result<(), ()> {
    let run_result = match exercise.mode {
        Mode::Compile => utils::compile_exercise(exercise)?,
        Mode::Execute => utils::execute_exercise(exercise)?,
        Mode::Test => utils::test_exercise(exercise)?,
    };
    utils::print_exercise_output(run_result);
    utils::print_exercise_success(exercise);
    Ok(())
}

// Resets the exercise by stashing the changes
pub fn reset(exercise: &Exercise) -> Result<(), ()> {
    let command = Command::new("git")
        .args(["stash", "--"])
        .arg(&exercise.path)
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
